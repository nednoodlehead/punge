use serde::{Deserialize, Serialize};

use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Cache {
    // represents the cached data from last session
    pub song_id: String,
    pub volume: f32,
    pub shuffle: bool,
    pub playlist: String,
}

pub fn fetch_cache() -> Cache {
    let file = std::fs::read_to_string("./cache/music.json").unwrap();
    let cache: Cache = serde_json::from_str(file.as_str()).unwrap();
    cache
}

pub fn dump_cache(data: Cache) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./cache/music.json")
        .unwrap();
    let serialized = serde_json::to_string(&data).unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}
