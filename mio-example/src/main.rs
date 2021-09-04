use mio::{Events, Interest, Poll, Token};
use mio::net::{TcpListener};
use std::io::{self};
use std::time::Duration;

fn main() -> io::Result<()> {
    println!("Metal IO (MIO) example");
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr = "127.0.0.1:9000".parse().unwrap();
    let mut listener = TcpListener::bind(addr)?;
    println!("Listening to {}", addr);
    const SERVER: Token = Token(0);
    poll.registry().register(&mut listener, SERVER, Interest::READABLE)?;

    loop {
        // Poll the OS for events, waiting at most 100 milliseconds.
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        // Process each event.
        for event in events.iter() {
            match event.token() {
                SERVER => loop {
                    match listener.accept() {
                        Ok((_connection, address)) => {
                            println!("Got a connection from: {}", address);
                        },
                        // A "would block error" is returned if the operation
                        // is not ready, so we'll stop trying to accept
                        // connections.
                        Err(ref err) if would_block(err) => break,
                        Err(err) => return Err(err),
                    }
                }
                Token(_) => {
                },
            }
        }
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

