use std::sync::Arc;

use anyhow::{Ok, Result, anyhow};
use server::stream::{Audio, DisplayControl, Inputs};
use shared::{
    codes::HidEvent,
    emulator::{HidEmulator, WinputEmulator},
};
use tokio::{join, net::TcpListener, select, sync::mpsc};

#[tokio::main]
async fn main() {
    let addr = "192.168.10.3:8080";
    let emulator = Arc::new(HidEmulator::new(0xa56, 0xa56, 1));
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
        let (wifi_rx, wifi_tx) = stream.into_split();
        let (display_tx, display_rx) = mpsc::channel::<()>(10);
        let inputs = Inputs::new(wifi_rx, emulator.clone(), display_tx);
        let audio = Audio::new(wifi_tx).unwrap();
        let inputs_handle = tokio::spawn(inputs.handle_loop());
        let audio_handle = tokio::spawn(audio.handle_loop());
        let display_handle = tokio::task::spawn_blocking(move || {
            let display_control = DisplayControl::new("G274QPF E2", display_rx);
            display_control.handle_loop();
        });
        let audio_handle_ab = audio_handle.abort_handle();
        let inputs_handle_ab = inputs_handle.abort_handle();
        let display_handle_ab = display_handle.abort_handle();
        select! {
            _ = inputs_handle => {
            },
            _ = audio_handle => {},
            _ = display_handle => {},
        };
        audio_handle_ab.abort();
        inputs_handle_ab.abort();
        display_handle_ab.abort();
    }
}
