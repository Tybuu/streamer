use server::stream::{Audio, Inputs};
use shared::codes::HidEvent;
use tokio::{join, net::TcpListener};

#[tokio::main]
async fn main() {
    let addr = "192.168.10.3:8080";
    let listener = TcpListener::bind(addr).await.unwrap();
    let (stream, _) = listener.accept().await.unwrap();
    stream.set_nodelay(true).unwrap();
    let (rx, tx) = stream.into_split();

    let inputs = Inputs::new(rx);
    let audio = Audio::new(tx);
    let inputs_handle = tokio::spawn(inputs.handle_loop());
    let audio_handle = tokio::spawn(audio.handle_loop());
    join!(inputs_handle, audio_handle);
}
