//
// Holds all torrent file related information
//

use serde_bytes::ByteBuf;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct File {
    path: Vec<String>,
    length: i64,
    #[serde(default)]
    md5sum: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
    #[serde(rename = "piece length")]
    piece_length: i64,
    pieces: ByteBuf,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    files: Option<Vec<File>>,
}

#[derive(Debug, Deserialize)]
pub struct Torrent {
    pub info: Info,
    #[serde(default)]
    pub announce: Option<String>,
}
