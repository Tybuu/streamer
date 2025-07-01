use bytemuck::checked::cast_slice;
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Observer, Producer, Split};
use std::collections::VecDeque;
use std::error;
use std::fs::File;
use std::io::Write;
use std::net::TcpListener;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, mpsc};
use std::thread;
use wasapi::*;

#[macro_use]
extern crate log;
use simplelog::*;

type Res<T> = Result<T, Box<dyn error::Error>>;

static START: AtomicBool = AtomicBool::new(false);

// Main loop
fn main() -> Res<()> {
    let _ = SimpleLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::new()
            .set_time_format_rfc3339()
            .set_time_offset_to_local()
            .unwrap()
            .build(),
    );

    initialize_mta().ok()?;

    let (mut tx, mut rx) = HeapRb::new((128 * 4) * 16).split();
    // Playback
    let _handle = thread::Builder::new()
        .name("Player".to_string())
        .spawn(move || {
            // let addr = "192.168.10.3:8080";
            // let listener = TcpListener::bind(addr).expect("Failed to bind to address");
            let mut file = File::create("data.bin").unwrap();
            let mut buf = [0u8; 128 * 4];
            START.store(true, std::sync::atomic::Ordering::Relaxed);
            let mut count = 0;
            while count < buf.len() {
                if !rx.is_empty() {
                    count += rx.pop_slice(&mut buf[count..]);
                }
            }
            file.write_all(&buf).unwrap();
            exit(0);
        });

    // Capture
    let _handle = thread::Builder::new()
        .name("Capture".to_string())
        .spawn(move || {
            let device = get_default_device(&Direction::Capture).unwrap();
            let mut audio_client = device.get_iaudioclient().unwrap();

            let desired_format = WaveFormat::new(32, 32, &SampleType::Float, 44100, 2, None);

            let blockalign = desired_format.get_blockalign();
            debug!("Desired capture format: {:?}", desired_format);

            let (def_time, min_time) = audio_client.get_device_period().unwrap();
            debug!("default period {}, min period {}", def_time, min_time);

            let mode = StreamMode::EventsShared {
                autoconvert: true,
                buffer_duration_hns: min_time,
            };
            audio_client
                .initialize_client(&desired_format, &Direction::Capture, &mode)
                .unwrap();
            debug!("initialized capture");

            let h_event = audio_client.set_get_eventhandle().unwrap();

            let buffer_frame_count = audio_client.get_buffer_size().unwrap();

            let render_client = audio_client.get_audiocaptureclient().unwrap();
            let mut buf = [0u8; 3600];
            audio_client.start_stream().unwrap();
            loop {
                let read = render_client.read_from_device(&mut buf).unwrap();
                if START.load(std::sync::atomic::Ordering::Relaxed) {
                    tx.push_slice(&buf[..(read.0 * 8) as usize]);
                    let f_slice: &[f32] = cast_slice(&buf[..(read.0 * 8) as usize]);
                    trace!("Data: {:?}", f_slice);
                }
                if h_event.wait_for_event(1000000).is_err() {
                    error!("error, stopping capture");
                    audio_client.stop_stream().unwrap();
                    break;
                }
            }
        });
    _handle.unwrap().join().unwrap();
    Ok(())
}
