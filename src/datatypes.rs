use serde_json;
use std::io;
use std::string::FromUtf8Error;


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
    Custom(String),
    Io(io::Error),
    Json(serde_json::Error),
    Utf8(FromUtf8Error)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Utf8(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}
