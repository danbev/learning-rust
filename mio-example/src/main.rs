//! MIO Example crate

use mio::{Poll, Token, Interest, event::Source, Events};
use mio::unix::SourceFd;
use std::os::unix::io::{AsRawFd, RawFd};
use std::io::{Read, BufRead};

/// main function for MIO example [`RawFd`][std::os::unix::io::RawFd]
pub fn main() {
    println!("Metal IO (MIO) example");

    let mut poll = match Poll::new() {
        Ok(poll) => poll,
        Err(e) => panic!("failed to create Poll instance; err={:?}", e),
    };
    // Get a ray pointer to stdin
    let raw_fd: RawFd = std::io::stdin().as_raw_fd();
    // Use the raw pointer to create a mutable refernce to a new SourceFd
    let source: &mut SourceFd =  &mut SourceFd(&raw_fd);

    let reg = poll.registry();
    let token = Token(0);
    // We can register the source using the registy:
    //let result = reg.register(source, token, Interest::READABLE);
    // Or we can register the source using the source itself, passing in the
    // registry:
    let result = source.register(reg, token, Interest::READABLE);
    match result {
        Ok(()) => println!("Registration successful!"),
        Err(e) => println!("Registration failed!")
    }
    // We use Events to retrieve the events we are interested in.
    let mut events = Events::with_capacity(1);
    println!("Going to poll for {} Events", events.capacity());

    let stdin = std::io::stdin();
    let mut iterator = stdin.lock().lines();
    let _line = iterator.next().unwrap();


    // So we have registered our interest in readable events for stdin, now
    // we need to get them. We want to poll for these events:
    let poll_result = poll.poll(&mut events, None);
    for event in &events {
        println!("event.token: {:?}", event.token());
    }
}

