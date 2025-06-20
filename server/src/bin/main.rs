use std::{io::Read, net::TcpListener};

use shared::codes::HidEvent;

fn main() {
    let addr = "192.168.10.3:8080";
    let listener = TcpListener::bind(addr).expect("Failed to bind to address");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => loop {
                let mut buf = [0; size_of::<usize>()];
                let buf_size = match stream.read_exact(&mut buf) {
                    Ok(()) => usize::from_le_bytes(buf),
                    Err(e) => panic!("{}", e),
                };
                let mut buffer = [0; 512];
                match stream.read_exact(&mut buffer[..buf_size]) {
                    Ok(()) => {
                        println!("Data: {:?}", &buffer[..buf_size]);
                        let event = bincode::deserialize::<HidEvent>(&buffer[..buf_size]).unwrap();
                        println!("Code: {:?}", event);
                        event.process_winput();
                    }
                    Err(e) => panic!("{}", e),
                }
            },
            Err(_) => todo!(),
        }
    }
}
