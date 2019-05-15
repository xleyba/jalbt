extern crate serde;
extern crate serde_bencode;
use serde_bencode::de;
use std::fs::File;
use std::io::Read;
mod torrent;
use torrent::Torrent;
//use serde_derive::{Serialize,Deserialize,Debug};

fn main() {
    let mut data =
        File::open("./Leinster - Saracens 11.05.19.mkv.torrent").expect("Unable to read file");
    let mut buffer = Vec::new();

    match data.read_to_end(&mut buffer) {
        Ok(_) => match de::from_bytes::<Torrent>(&buffer) {
            Ok(t) => {
                println!("name:\t\t{}", &t.info.name);
                match &t.announce {
                    Some(t) => println!("Announce: \t\t{}", t),
                    None => println!("Announce: \t\t"),
                };
            }
            Err(e) => println!("ERROR: {:?}", e),
        },
        Err(e) => println!("ERROR: {:?}", e),
    }
}
