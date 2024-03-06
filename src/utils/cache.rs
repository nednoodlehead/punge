use std::io::Write;

use crate::playliststructs::Config;
use serde_json;

pub fn read_from_cache() -> Result<Config, std::io::Error> {
    let file = std::fs::read_to_string("./cache/config.json")?;
    let config: Config = serde_json::from_str(file.as_str())?;
    Ok(config)
}

pub fn write_to_cache(config: Config) -> Result<(), std::io::Error> {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./cache/config.json")?;
    let serial = serde_json::to_string(&config)?;
    file.write_all(serial.as_bytes())?;
    Ok(())
}
