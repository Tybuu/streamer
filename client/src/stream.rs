use std::sync::Arc;
use std::time::Duration;

use bytemuck::cast_slice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, Stream, StreamConfig};
use ringbuf::traits::{Consumer, Observer, Producer, Split};
use ringbuf::wrap::caching::Caching;
use ringbuf::{CachingProd, HeapRb, SharedRb};
use shared::codes::{HidEvent, ScanCode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedReadHalf;
use tokio::{net::tcp::OwnedWriteHalf, sync::mpsc::Receiver};

type ChannelData = HidEvent;

pub struct Inputs {
    wifi_tx: OwnedWriteHalf,
    data_rx: Receiver<ChannelData>,
}

impl Inputs {
    pub fn new(wifi_tx: OwnedWriteHalf, data_rx: Receiver<ChannelData>) -> Self {
        Self { wifi_tx, data_rx }
    }

    async fn send_data(&mut self, data: &ChannelData) {
        let mesg = bincode::serialize(&data).unwrap();
        self.wifi_tx.write_u8(mesg.len() as u8).await.unwrap();
        self.wifi_tx.write_all(&mesg).await.unwrap();
    }

    pub async fn handle_loop(mut self) {
        self.send_data(&ChannelData::Key(ScanCode::new(
            winit::keyboard::KeyCode::NumLock,
            winit::event::ElementState::Pressed,
        )))
        .await;
        // tokio::time::sleep(Duration::from_millis(10)).await;
        // self.send_data(&ChannelData::Key(ScanCode::new(
        //     winit::keyboard::KeyCode::NumLock,
        //     winit::event::ElementState::Released,
        // )))
        // .await;

        loop {
            let key: ChannelData = self.data_rx.recv().await.unwrap();
            self.send_data(&key).await;
        }
    }
}

pub struct Audio {
    wifi_rx: OwnedReadHalf,
    audio_tx: CachingProd<Arc<HeapRb<f32>>>,
    stream: Stream,
}

impl Audio {
    pub fn new(wifi_rx: OwnedReadHalf) -> Self {
        let (audio_tx, mut consumer) = HeapRb::<f32>::new(44100).split();

        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("No default output device found");
        println!("Using audio device: {}", device.name().unwrap());

        let supported_configs_range = device.supported_output_configs().unwrap();
        let supported_config = supported_configs_range
            .filter(|c| c.sample_format() == cpal::SampleFormat::F32 && c.channels() == 2)
            .next()
            .expect("No supported output config found")
            .with_sample_rate(cpal::SampleRate(48000));

        println!("Supported config: {:?}", supported_config);
        let output_config: StreamConfig = supported_config.into();

        let config = StreamConfig {
            buffer_size: BufferSize::Fixed(256 * 2), // Or a smaller number like 256 or 128
            ..output_config
        };

        let data_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            if consumer.occupied_len() >= data.len() {
                consumer.pop_slice(data);
            } else {
                data.fill(0.0);
            }
        };

        let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

        let stream = device
            .build_output_stream(
                &config,
                data_callback,
                err_fn,
                None, // None means no timeout for stream creation
            )
            .unwrap();
        stream.play().unwrap();
        Self {
            wifi_rx,
            audio_tx,
            stream,
        }
    }

    pub async fn handle_loop(mut self) {
        let mut buf = [0u8; 128 * 2 * 4];
        loop {
            self.wifi_rx.read_exact(&mut buf).await.unwrap();
            let f_slice: &[f32] = cast_slice(&buf);
            self.audio_tx.push_slice(f_slice);
        }
    }
}
