use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use bytemuck::cast_slice;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, StreamConfig};
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Producer, Split}; // For easy error handling

fn main() {
    // 1. Get the default audio host
    // A host provides access to the audio devices on the system.
    let host = cpal::default_host();

    // 2. Get the default output device
    // We'll typically use the system's default output device.
    let device = host
        .default_output_device()
        .expect("No default output device found");

    println!("Using audio device: {}", device.name().unwrap());

    // 3. Get a supported output configuration
    // Devices can support various sample rates, channel counts, and sample formats.
    // We'll try to get the highest sample rate supported for simplicity.

    let supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range
        .filter(|c| c.sample_format() == cpal::SampleFormat::F32 && c.channels() == 2)
        .next()
        .expect("No supported output config found")
        .with_sample_rate(cpal::SampleRate(44100)); // Or choose a specific rate like .with_sample_rate(cpal::SampleRate(44100))

    println!("Supported config: {:?}", supported_config);

    // 4. Convert the supported config into a usable `StreamConfig`
    // We'll generally use f32 for floating-point audio processing,
    // as it's common in DSP and provides good precision.
    let sample_format = supported_config.sample_format();
    let output_config: StreamConfig = supported_config.into();

    // For very low latency, you might try a fixed buffer size:
    let config = StreamConfig {
        buffer_size: BufferSize::Fixed(256 * 2), // Or a smaller number like 256 or 128
        ..output_config
    };
    // Be aware that very small buffers can cause underruns (crackling) if your CPU can't keep up.

    // 5. Create a `Stream`
    // This is where the magic happens: you provide a callback function that `cpal` will call
    // whenever it needs more audio data.
    let sample_rate = output_config.sample_rate.0 as f32;
    let channels = output_config.channels as usize;

    println!("Config: {:?}", config);

    let (mut producer, mut consumer) = HeapRb::<f32>::new(44100).split();

    let input_data = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        for frame in data.chunks(channels) {
            producer.push_slice(frame);
        }
    };

    // Define an error callback function
    let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

    let input_stream = device
        .build_input_stream(&config, input_data, err_fn, None)
        .unwrap();

    let mut file = File::create("data.bin").unwrap();
    // 6. Start the stream
    input_stream.play().unwrap();
    let mut buf = [0f32; 512];
    loop {
        let res = consumer.pop_slice(&mut buf);
        if res != 0 {
            file.write(cast_slice(&buf[..res])).unwrap();
        }
    }
}
