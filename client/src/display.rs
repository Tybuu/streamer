use std::{fs, path::Path};

use ddc_hi::{Ddc, Display};
use shared::codes::ChannelData;
use tokio::{net::UnixDatagram, sync::mpsc::Sender};

pub struct DisplayControl {
    sock: UnixDatagram,
    display: Display,
    shared_tx: Sender<Vec<u8>>,
}

impl DisplayControl {
    pub fn new<P: AsRef<Path>>(
        bind_path: P,
        display_name: &str,
        shared_tx: Sender<Vec<u8>>,
    ) -> Self {
        fs::remove_file(bind_path.as_ref());
        let sock = UnixDatagram::bind(bind_path).unwrap();

        let display = Display::enumerate()
            .into_iter()
            .find(|x| x.info.model_name.as_ref().unwrap() == display_name)
            .unwrap();

        Self {
            sock,
            display,
            shared_tx,
        }
    }

    pub async fn handle_loop(mut self) {
        const DISPLAY_OUTPUT_CODE: u8 = 0x60;
        const HDMI2: u16 = 0x12;
        loop {
            // Any external program can signal to switch displays by writing to the unix datagram
            // The internal data doesn't matter
            self.sock.recv(&mut [0u8]).await.unwrap();

            // Getting any vcp feature when the corresponding output of the monitor isn't
            // on this computer returns an error so we can use this fact to determine which
            // computer needs to write to the monitor to switch
            match self.display.handle.get_vcp_feature(DISPLAY_OUTPUT_CODE) {
                Ok(_) => {
                    self.display
                        .handle
                        .set_vcp_feature(DISPLAY_OUTPUT_CODE, HDMI2);
                }
                Err(_) => {
                    let buf = bincode::serialize(&ChannelData::ChangeDisplay).unwrap();
                    self.shared_tx.send(buf).await.unwrap();
                }
            }
        }
    }
}
