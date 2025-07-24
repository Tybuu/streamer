use std::sync::Arc;

use anyhow::Result;
use bytemuck::cast_slice;
use cpal::{
    BufferSize, Stream, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use ddc_hi::{Ddc, Display};
use ringbuf::{
    CachingCons, HeapRb,
    traits::{Consumer, Producer, Split},
};
use shared::{
    codes::{ChannelData, HidEvent},
    emulator::{Emulator, WinputEmulator},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::{
        Mutex,
        mpsc::{Receiver, Sender},
    },
};

pub struct Inputs<E: Emulator> {
    wifi_rx: OwnedReadHalf,
    emulator: Arc<E>,
    display_tx: Sender<()>,
}

impl<E: Emulator> Inputs<E> {
    pub fn new(wifi_rx: OwnedReadHalf, emulator: Arc<E>, display_tx: Sender<()>) -> Self {
        Self {
            wifi_rx,
            emulator,
            display_tx,
        }
    }

    pub async fn handle_loop(mut self) -> Result<()> {
        let mut buf = vec![0u8; 32];
        loop {
            let size = self.wifi_rx.read_u8().await? as usize;
            self.wifi_rx.read_exact(&mut buf[..size]).await?;
            let event = bincode::deserialize::<ChannelData>(&buf[..size])?;
            match event {
                ChannelData::Hid(hid_event) => {
                    self.emulator.emulate_input(&hid_event);
                }
                ChannelData::ChangeDisplay => {
                    self.display_tx.send(()).await?;
                }
            }
        }
    }
}

pub struct Audio {
    wifi_tx: OwnedWriteHalf,
    audio_rx: CachingCons<Arc<HeapRb<f32>>>,
    stream: Stream,
}

impl Audio {
    pub fn new(wifi_tx: OwnedWriteHalf) -> Result<Self> {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("No default output device found");

        println!("Using audio device: {}", device.name()?);

        let supported_configs_range = device.supported_output_configs()?;
        let supported_config = supported_configs_range
            .filter(|c| c.sample_format() == cpal::SampleFormat::F32 && c.channels() == 2)
            .next()
            .expect("No supported output config found")
            .with_sample_rate(cpal::SampleRate(48000)); // Or choose a specific rate like .with_sample_rate(cpal::SampleRate(44100))

        println!("Supported config: {:?}", supported_config);

        let sample_format = supported_config.sample_format();
        let output_config: StreamConfig = supported_config.into();

        // For very low latency, you might try a fixed buffer size:
        let config = StreamConfig {
            buffer_size: BufferSize::Fixed(128 * 2), // Or a smaller number like 256 or 128
            ..output_config
        };

        println!("Config: {:?}", config);

        let (mut producer, audio_rx) = HeapRb::<f32>::new(128 * 2 * 10).split();

        let input_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            producer.push_slice(data);
        };

        // Define an error callback function
        let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

        let input_stream = device.build_input_stream(&config, input_data, err_fn, None)?;
        input_stream.play()?;
        Ok(Self {
            wifi_tx,
            audio_rx,
            stream: input_stream,
        })
    }

    pub async fn handle_loop(mut self) -> Result<()> {
        let mut buf = [0f32; 128 * 2];
        loop {
            let mut count = 0;
            while count < buf.len() {
                count += self.audio_rx.pop_slice(&mut buf[count..]);
            }
            self.wifi_tx.write_all(cast_slice(&buf)).await?;
        }
    }
}
pub struct DisplayControl {
    display: Display,
    rx: Receiver<()>,
}

impl DisplayControl {
    pub fn new(display_name: &str, rx: Receiver<()>) -> Self {
        let display = Display::enumerate()
            .into_iter()
            .find(|x| match x.info.model_name.as_ref() {
                Some(val) => val == display_name,
                None => false,
            })
            .unwrap();
        Self { display, rx }
    }

    pub async fn handle_loop(mut self) {
        loop {
            if self.rx.recv().await.is_none() {
                break;
            }
            self.display.handle.set_vcp_feature(0x60, 0x10);
        }
    }
}
