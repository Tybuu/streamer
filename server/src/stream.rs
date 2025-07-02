use std::sync::Arc;

use bytemuck::cast_slice;
use cpal::{
    BufferSize, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use ringbuf::{
    CachingCons, HeapRb,
    traits::{Consumer, Producer, Split},
};
use shared::codes::HidEvent;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

pub struct Inputs {
    wifi_rx: OwnedReadHalf,
}

impl Inputs {
    pub fn new(wifi_rx: OwnedReadHalf) -> Self {
        Self { wifi_rx }
    }

    pub async fn handle_loop(mut self) {
        let mut buf = [0u8; size_of::<HidEvent>()];
        loop {
            self.wifi_rx.read_exact(&mut buf).await.unwrap();
            let event = bincode::deserialize::<HidEvent>(&buf).unwrap();
            event.process_winput();
        }
    }
}

pub struct Audio {
    wifi_tx: OwnedWriteHalf,
    audio_rx: CachingCons<Arc<HeapRb<f32>>>,
}

impl Audio {
    pub fn new(wifi_tx: OwnedWriteHalf) -> Self {
        let host = cpal::default_host();

        let device = host
            .default_input_device()
            .expect("No default output device found");

        println!("Using audio device: {}", device.name().unwrap());

        let supported_configs_range = device.supported_output_configs().unwrap();
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

        let (mut producer, mut audio_rx) = HeapRb::<f32>::new(128 * 2 * 10).split();

        let input_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            producer.push_slice(data);
        };

        // Define an error callback function
        let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

        let input_stream = device
            .build_input_stream(&config, input_data, err_fn, None)
            .unwrap();
        input_stream.play().unwrap();
        Self { wifi_tx, audio_rx }
    }

    pub async fn handle_loop(mut self) {
        let mut buf = [0f32; 128 * 2];
        loop {
            let mut count = 0;
            while count < buf.len() {
                count += self.audio_rx.pop_slice(&mut buf[count..]);
            }
            self.wifi_tx.write_all(cast_slice(&buf)).await.unwrap();
        }
    }
}
