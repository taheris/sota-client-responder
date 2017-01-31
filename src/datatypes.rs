use serde_json;
use std::io;


#[derive(Deserialize, Debug)]
pub struct Event {
    pub version: String,
    pub event:   String,
    pub data:    DownloadComplete
}

#[derive(Deserialize, Debug)]
pub struct DownloadComplete {
    pub update_id:    String,
    pub update_image: String,
    pub signature:    String
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
