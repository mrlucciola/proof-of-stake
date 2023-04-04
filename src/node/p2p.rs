// imports
use std::{
    io,
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    time::Duration,
};
// local
use super::error::P2PError;
// submodule
pub type Result<T> = std::result::Result<T, P2PError>;

#[derive(Debug)]
pub struct P2P {
    host: IpAddr,
    port: u16,
    socket_addr: SocketAddr,
}

fn handle_stream(stream: TcpStream) -> std::result::Result<(), P2PError> {
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
            Ok() => {
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
    /// ## Initialize a new peer-to-peer connection
    ///
    /// Get environment information from a config file.
    pub fn new(host: IpAddr, port: u16) -> Self {
        let socket_addr = SocketAddr::new(host, port);

        Self {
            host,
            port,
            socket_addr,
        }
    }
    pub fn start_connection(&self) -> Result<()> {
        let listener = TcpListener::bind(self.socket_addr)?;
        for stream_res in listener.incoming() {
            let stream = stream_res?;
            handle_stream(stream)?;
        }

        Ok(())
    }
}
