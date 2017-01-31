use serde_json;
use std::io;


#[derive(Deserialize, Debug)]
pub struct Event {
    version: String,
    event:   String,
    data:    EventType
}

#[derive(Deserialize, Debug)]
pub enum EventType {
    DownloadComplete {
        update_id:    String,
        update_image: String,
        signature:    String
    },
    DownloadFailed {
        update_id: String,
        reason:    String
    }
}


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
