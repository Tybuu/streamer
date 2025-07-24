use client::app::App;
use client::display::DisplayControl;
use client::stream::{Audio, Inputs, SharedSender};
use shared::codes::HidEvent;
use std::thread;
use tokio::join;
use tokio::net::TcpStream;
use tokio::runtime::Builder;
use tokio::sync::mpsc::{self, channel};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn main() {
    let (hid_tx, hid_rx) = channel(128);

    thread::spawn(move || {
        let rt = Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async {
            let addr = "192.168.10.3:8080";
            let stream;
            loop {
                let new_stream = TcpStream::connect(addr).await;
                match new_stream {
                    Ok(res) => {
                        stream = res;
                        break;
                    }
                    Err(_) => {}
                }
            }
            stream.set_nodelay(true).unwrap();
            let (wifi_rx, wifi_tx) = stream.into_split();

            let (write_tx, write_rx) = mpsc::channel::<Vec<u8>>(20);

            let shared_sender = SharedSender::new(wifi_tx, write_rx);

            let inputs = Inputs::new(write_tx.clone(), hid_rx);
            let audio = Audio::new(wifi_rx);
            let display_control = DisplayControl::new("/tmp/stream_temp", "G274QPF E2", write_tx);

            let shared_handle = tokio::spawn(shared_sender.write_loop());
            let display_handle = tokio::spawn(display_control.handle_loop());
            let input_handle = tokio::spawn(inputs.handle_loop());
            let audio_handle = tokio::spawn(audio.handle_loop());
            join!(shared_handle, input_handle, display_handle, audio_handle);
        });
    });
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new(hid_tx);
    event_loop.run_app(&mut app).unwrap();
}
