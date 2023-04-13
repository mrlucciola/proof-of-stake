mod types;
// external
use std::{
    io::{self, Read},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    time::Duration,
};
// local
use super::{error::P2PError, types::Result};

#[derive(Debug)]
pub struct P2P {
    pub host: IpAddr,
    pub port: u16,
    pub socket_addr: SocketAddr,
    listener: TcpListener,
}

fn handle_stream(mut stream: TcpStream) -> std::result::Result<(), P2PError> {
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
    /// ## Initialize a new peer-to-peer connection
    ///
    /// Get environment information from a config file.
    pub fn new(host: IpAddr, port: u16) -> Self {
        let socket_addr = SocketAddr::new(host, port);
        let listener = TcpListener::bind(socket_addr).unwrap();

        Self {
            host,
            port,
            socket_addr,
            listener,
        }
    }
    pub fn start_connection(&self) -> Result<()> {
        // self.listener.set_nonblocking(true)?;

        for stream_res in self.listener.incoming() {
            match stream_res {
                Ok(s) => handle_stream(s)?,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => panic!("Unknown error: {}", e),
            }
        }

        Ok(())
    }
    /// ### Start the p2p connection.
    pub fn start_p2p(&mut self) -> Result<()> {
        // start the connection
        self.start_connection()?;

        Ok(())
    }
}
