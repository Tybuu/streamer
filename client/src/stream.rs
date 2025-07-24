use std::sync::Arc;
use std::time::Duration;

use bytemuck::cast_slice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device, Stream, StreamConfig};
use ringbuf::traits::{Consumer, Observer, Producer, Split};
use ringbuf::wrap::caching::Caching;
use ringbuf::{CachingProd, HeapRb, SharedRb};
use serde::Serialize;
use shared::codes::{ChannelData, HidEvent, ScanCode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedReadHalf;
use tokio::sync::mpsc::Sender;
use tokio::{net::tcp::OwnedWriteHalf, sync::mpsc::Receiver};

pub struct Inputs {
    shared_tx: Sender<Vec<u8>>,
    data_rx: Receiver<HidEvent>,
}

impl Inputs {
    pub fn new(shared_tx: Sender<Vec<u8>>, data_rx: Receiver<HidEvent>) -> Self {
        Self { shared_tx, data_rx }
    }

    pub async fn handle_loop(mut self) {
        loop {
            let key: HidEvent = self.data_rx.recv().await.unwrap();
            let mesg = bincode::serialize(&ChannelData::Hid(key)).unwrap();
            self.shared_tx.send(mesg).await.unwrap();
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

pub struct SharedSender {
    writer: OwnedWriteHalf,
    rx: Receiver<Vec<u8>>,
}
impl SharedSender {
    pub fn new(writer: OwnedWriteHalf, rx: Receiver<Vec<u8>>) -> Self {
        Self { writer, rx }
    }

    pub async fn write_loop(mut self) {
        loop {
            match self.rx.recv().await {
                Some(buf) => {
                    self.writer.write_u8(buf.len() as u8).await.unwrap();
                    self.writer.write_all(&buf).await.unwrap();
                }
                None => {
                    break;
                }
            }
        }
    }
}
