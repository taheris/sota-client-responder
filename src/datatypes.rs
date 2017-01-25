use serde::de::{Deserialize, Deserializer, Error as SerdeError, MapVisitor, Visitor};
use serde::de::value::MapVisitorDeserializer;


#[derive(Deserialize)]
struct Event {
    version: String,
    event:   String,
    data:    EventType
}

#[derive(Deserialize)]
enum EventType {
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

enum Error {
}
