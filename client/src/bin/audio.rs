use std::{io::Read, net::TcpStream, sync::mpsc};

use cpal::{
    BufferSize, StreamConfig,
    traits::{DeviceTrait, HostTrait},
};
use ringbuf::{
    HeapRb,
    traits::{Consumer, Producer, Split},
};

fn main() {
    let (mut producer, mut consumer) = HeapRb::<u8>::new(44100).split();
    let socket_thread = std::thread::spawn(move || {
        let addr = "192.168.10.3:8080";
        let mut stream = TcpStream::connect(addr).expect("Unable to connect to stream");
        stream.set_nodelay(true).unwrap();
        let mut buf = [0u8; 2048];
        loop {
            let data = stream.read(&mut buf).unwrap();
            producer.push_slice(&buf[..data]);
        }
    });

    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("No default output device found");

    println!("Using audio device: {}", device.name()?);

    let supported_configs_range = device.supported_output_configs()?;
    let supported_config = supported_configs_range
        .filter(|c| c.sample_format() == cpal::SampleFormat::U8 && c.channels() == 2)
        .next()
        .expect("No supported output config found")
        .with_sample_rate(cpal::SampleRate(44100)); // Or choose a specific rate like .with_sample_rate(cpal::SampleRate(44100))

    println!("Supported config: {:?}", supported_config);

    let sample_format = supported_config.sample_format();
    let output_config: StreamConfig = supported_config.into();

    let config = StreamConfig {
        buffer_size: BufferSize::Fixed(256 * 2), // Or a smaller number like 256 or 128
        ..output_config
    };

    let sample_rate = output_config.sample_rate.0 as f32;
    let channels = output_config.channels as usize;

    let data_callback = move |data: &mut [u8], _: &cpal::OutputCallbackInfo| {
        for frame in data.chunks_mut(channels) {
            let res = consumer.pop_slice(frame);
            frame[res..frame.len()].fill(0);
        }
    };

    let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

    let stream = device.build_output_stream(
        &config,
        data_callback,
        err_fn,
        None, // None means no timeout for stream creation
    )?;

    socket_thread.join();
}
