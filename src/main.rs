extern crate core;

use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::sync::{mpsc, mpsc::Receiver};
mod playliststructs;

mod db;
mod gui;
mod player;
mod utils;

fn main() {
    gui::start::begin().unwrap();
    // println!(
    //     "{:?}",
    //     player::sort::get_values_from_db("main".to_string(), "kenlammortal".to_string())
    // );
}
