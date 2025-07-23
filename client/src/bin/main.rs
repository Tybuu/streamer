use client::app::App;
use client::stream::{Audio, Inputs};
use shared::codes::HidEvent;
use std::thread;
use tokio::join;
use tokio::net::TcpStream;
use tokio::runtime::Builder;
use tokio::sync::mpsc::channel;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn main() {
    let (tx, rx) = channel(128);

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

            let inputs = Inputs::new(wifi_tx, rx);
            let audio = Audio::new(wifi_rx);
            let input_handle = tokio::spawn(inputs.handle_loop());
            let audio_handle = tokio::spawn(audio.handle_loop());
            join!(input_handle, audio_handle);
        });
    });
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::new(tx);
    event_loop.run_app(&mut app).unwrap();
}
