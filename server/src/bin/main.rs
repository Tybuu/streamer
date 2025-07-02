use anyhow::{Ok, Result, anyhow};
use server::stream::{Audio, Inputs};
use shared::codes::HidEvent;
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
        let inputs = Inputs::new(rx);
        let audio = Audio::new(tx).unwrap();
        select! {
            _ = inputs.handle_loop() => {},
            _ = audio.handle_loop() => {}
        };
    }
}
