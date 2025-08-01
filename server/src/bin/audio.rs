use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

use bytemuck::cast_slice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, StreamConfig};
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Producer, Split}; // For easy error handling

fn main() {
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

    let sample_rate = output_config.sample_rate.0 as f32;
    let channels = output_config.channels as usize;

    println!("Config: {:?}", config);

    let (mut producer, mut consumer) = HeapRb::<f32>::new(128 * 2 * 10).split();

    let input_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        producer.push_slice(data);
    };

    // Define an error callback function
    let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

    let input_stream = device
        .build_input_stream(&config, input_data, err_fn, None)
        .unwrap();

    let addr = "192.168.10.3:8080";
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connected");
                input_stream.play().unwrap();
                let mut buf = [0f32; 128 * 2];
                loop {
                    let mut count = 0;
                    while count < buf.len() {
                        count += consumer.pop_slice(&mut buf[count..]);
                    }
                    stream.write_all(cast_slice(&buf)).unwrap();
                }
            }
            Err(_) => todo!(),
        }
    }
}
