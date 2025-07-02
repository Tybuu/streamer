use std::{io::Read, net::TcpStream, sync::mpsc};

use bytemuck::cast_slice;
use cpal::{
    BufferSize, StreamConfig,
    traits::{DeviceTrait, HostTrait},
};
use ringbuf::{
    HeapRb,
    traits::{Consumer, Observer, Producer, Split},
};

fn main() {
    let (mut producer, mut consumer) = HeapRb::<f32>::new(44100).split();
    let socket_thread = std::thread::spawn(move || {
        println!("Connecting to stream");
        let addr = "192.168.10.3:8080";
        let mut stream = TcpStream::connect(addr).expect("Unable to connect to stream");
        stream.set_nodelay(true).unwrap();
        let mut buf = [0u8; 128 * 2 * 4];
        println!("Connected");
        loop {
            let data = stream.read_exact(&mut buf).unwrap();
            let f_slice: &[f32] = cast_slice(&buf);
            producer.push_slice(f_slice);
        }
    });

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
    let sample_format = supported_config.sample_format();
    let output_config: StreamConfig = supported_config.into();

    let config = StreamConfig {
        buffer_size: BufferSize::Fixed(256 * 2), // Or a smaller number like 256 or 128
        ..output_config
    };

    let sample_rate = output_config.sample_rate.0 as f32;
    let channels = output_config.channels as usize;

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

    socket_thread.join().unwrap();
}
