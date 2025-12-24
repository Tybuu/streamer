use client::app::App;
use client::display::DisplayControl;
use client::stream::{Audio, Inputs, SharedSender};
use shared::codes::HidEvent;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::thread;
use tokio::join;
use tokio::net::TcpStream;
use tokio::runtime::Builder;
use tokio::sync::mpsc::{self, channel};
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
    let mac_address: [u8; 6] = [0xD8, 0x5E, 0xD3, 0x85, 0x95, 0xEB];
    wake_computer(mac_address).unwrap();
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

fn wake_computer(mac_address: [u8; 6]) -> std::io::Result<()> {
    let mut packet = Vec::with_capacity(102);
    packet.extend_from_slice(&[0xFF; 6]);
    for _ in 0..16 {
        packet.extend_from_slice(&mac_address);
    }

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;

    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::from_str("192.168.10.255").unwrap(), 9);

    socket.send_to(&packet, broadcast_addr)?;
    Ok(())
}
