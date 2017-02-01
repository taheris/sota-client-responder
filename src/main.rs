extern crate env_logger;
#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate unix_socket;

mod datatypes;

use datatypes::{Error, Event};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, ErrorKind, Write};
use std::process::Command;
use unix_socket::{UnixListener, UnixStream};


fn main() {
    env_logger::init().unwrap();

    let socket_path = env::args().nth(1).expect("first argument must be events socket path");
    let device_path = env::args().nth(2).expect("second argument must be device path");

    let events_socket = match UnixListener::bind(&socket_path) {
        Ok(socket) => socket,

        Err(ref err) if err.kind() == ErrorKind::AddrInUse => {
            fs::remove_file(&socket_path).expect("couldn't remove socket file");
            UnixListener::bind(&socket_path).expect("couldn't reopen events socket")
        }

        Err(err) => panic!("couldn't open events socket: {}", err)
    };

    let mut device = OpenOptions::new()
        .write(true)
        .open(device_path)
        .expect("couldn't open device");

    for conn in events_socket.incoming() {
        if let Err(err) = conn {
            error!("couldn't read events socket: {}", err);
            continue
        }

        let mut stream = conn.unwrap();
        let _ = read_event(&mut stream)
            .and_then(deb_description)
            .and_then(|desc| write_to_device(&mut device, &desc))
            .map_err(|err| error!("{:?}", err));
    }
}

fn read_event(stream: &mut UnixStream) -> Result<Event, Error> {
    debug!("new connection");
    let reader = BufReader::new(stream);
    Ok(serde_json::from_reader(reader)?)
}

fn deb_description(event: Event) -> Result<String, Error> {
    debug!("reading deb description for event: {:?}", event);
    let output = Command::new("dpkg-deb")
        .arg("-f")
        .arg(event.data.update_image)
        .arg("Description")
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

fn write_to_device(device: &mut File, text: &str) -> Result<(), Error> {
    debug!("writing to device: {}", text);
    Ok(device.write_all(text.as_bytes())?)
}
