#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate unix_socket;

mod datatypes;

use datatypes::{Error, Event};
use std::env;
use std::io::{BufReader, Read};
use unix_socket::{UnixListener, UnixStream};


fn main() {
    let socket_path   = env::args().nth(1).expect("1st argument must be events socket path");
    let events_socket = UnixListener::bind(&socket_path).expect("couldn't open events socket");

    for conn in events_socket.incoming() {
        if let Err(err) = conn {
            error!("couldn't read events socket: {}", err);
            continue
        }

        let mut stream = conn.unwrap();
        let _ = read_event(&mut stream)
            .map(|ev| debug!("event: {:?}", ev))
            .map_err(|err| error!("couldn't read event: {:?}", err));
    }
}

fn read_event(stream: &mut UnixStream) -> Result<Event, Error> {
    info!("New socket connection");
    let mut reader = BufReader::new(stream);
    let mut input  = String::new();
    reader.read_to_string(&mut input)?;
    debug!("socket input: {}", input);
    Ok(serde_json::from_str(&input)?)
}
