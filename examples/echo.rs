extern crate miofib;

use std::net::SocketAddr;
use std::str::FromStr;
use std::io::{self, Read, Write};
use miofib::net::TcpListener;

const DEFAULT_LISTEN_ADDR : &'static str = "127.0.0.1:5555";

fn listend_addr() -> SocketAddr {
    FromStr::from_str(DEFAULT_LISTEN_ADDR).unwrap()
}

macro_rules! printerrln {
    ($($arg:tt)*) => ({
        use std::io::prelude::*;
        if let Err(e) = writeln!(&mut ::std::io::stderr(), "{}",
            format_args!($($arg)*)) {
            panic!(concat!(
                    "Failed to write to stderr.\n",
                    "Original error output: {}\n",
                    "Secondary error writing to stderr: {}"),
                    format_args!($($arg)*), e);
        }
    })
}

fn main() {
    let _join = miofib::spawn(|| -> io::Result<()> {
        let addr = listend_addr();

        let listener = TcpListener::bind(&addr)?;

        printerrln!("Starting tcp echo server on {:?}", listener.local_addr()?);

        loop {
            let (mut conn, _addr) = listener.accept()?;

            let _join = miofib::spawn(move || -> io::Result<()> {
                let mut buf = vec![0u8; 1024 * 8];
                loop {
                    let size = conn.read(&mut buf)?;
                    if size == 0 {/* eof */ break; }
                    let _ = conn.write_all(&mut buf[0..size])?;
                }

                Ok(())
            });
        }
    }).join().unwrap();
}
