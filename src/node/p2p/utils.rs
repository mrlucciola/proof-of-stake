// external
use std::{
    io::{self, Read},
    net::TcpStream,
    time::Duration,
};
// local
use super::{error::P2PError, types::Result, P2P};

#[deprecated(note = "To be refactored for libp2p.")]
#[allow(dead_code)]
pub fn handle_stream(mut stream: TcpStream) -> std::result::Result<(), P2PError> {
    let mut buf = [0];
    stream.set_read_timeout(Some(Duration::from_millis(100)))?;

    loop {
        let _ = match stream.read(&mut buf) {
            Err(e) => match e.kind() {
                io::ErrorKind::WouldBlock => {
                    println!("Would have blocked");
                    break;
                }
                _ => panic!("Got an error"),
            },
            Ok(m) => {
                println!("Received {m:?}, {buf:?}");
                if m == 0 {
                    break;
                };
                m
            }
        };
    }

    Ok(())
}

impl P2P {
    #[deprecated(note = "To be refactored for libp2p.")]
    pub fn start_connection(&self) -> Result<()> {
        // self.listener.set_nonblocking(true)?;

        // for stream_res in self.listener().incoming() {
        //     match stream_res {
        //         Ok(s) => handle_stream(s)?,
        //         Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
        //             break;
        //         }
        //         Err(e) => panic!("Unknown error: {}", e),
        //     }
        // }

        Ok(())
    }
    /// ### Start the p2p connection.
    #[deprecated(note = "To be refactored for libp2p.")]
    pub fn start_p2p(&mut self) -> Result<()> {
        // start the connection
        // self.start_connection()?;

        Ok(())
    }
}
