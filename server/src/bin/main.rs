use anyhow::{Ok, Result, anyhow};
use server::stream::{Audio, Inputs};
use shared::{
    codes::HidEvent,
    emulator::{HidEmulator, WinputEmulator},
};
use tokio::{join, net::TcpListener, select};

#[tokio::main]
async fn main() {
    let addr = "192.168.10.3:8080";
    loop {
        let listener = TcpListener::bind(addr).await;
        let listener = if listener.is_ok() {
            unsafe { listener.unwrap_unchecked() }
        } else {
            continue;
        };
        let stream = listener.accept().await;
        let (stream, _) = if stream.is_ok() {
            unsafe { stream.unwrap_unchecked() }
        } else {
            continue;
        };
        stream.set_nodelay(true).unwrap();
        let (rx, tx) = stream.into_split();
        let emulator = HidEmulator::new(0xa56, 0xa56, 1);
        let inputs = Inputs::new(rx, emulator);
        let audio = Audio::new(tx).unwrap();
        let inputs_handle = tokio::spawn(inputs.handle_loop());
        let audio_handle = tokio::spawn(audio.handle_loop());
        let audio_handle_ab = audio_handle.abort_handle();
        let inputs_handle_ab = inputs_handle.abort_handle();
        select! {
            _ = inputs_handle => {
            },
            _ = audio_handle => {}
        };
        audio_handle_ab.abort();
        inputs_handle_ab.abort();
    }
}
