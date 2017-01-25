#[macro_use] extern crate log;
extern crate serde;
extern crate serde_json;
extern crate unix_socket;

use std::env;
use std::io::{BufReader, Read, Write};
use unix_socket::{UnixListener, UnixStream};

mod datatypes;


fn main() {
    let socket_path   = env::args().nth(1).expect("1st argument must be events socket path");
    let events_socket = UnixListener::bind(&socket_path).expect("couldn't open events socket");

    for conn in events_socket.incoming() {
        if let Err(err) = conn {
            error!("couldn't read events socket: {}", err);
            continue
        }

        let mut stream = conn.unwrap();
        let event = read_event(&mut stream);
    }
}

fn read_event<T>(stream: &mut UnixStream) -> Result<Event<T>, Error>
    where T: Decodable + Encodable
{
    info!("New socket connection");
    let mut reader = BufReader::new(stream);
    let mut input  = String::new();
    reader.read_to_string(&mut input)?;
    debug!("socket input: {}", input);
    input.parse()
}
