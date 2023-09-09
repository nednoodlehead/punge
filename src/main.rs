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
    // gui::start::begin().unwrap();
    let x = utils::db_convert::get_duration(String::from(
        r"F:\Projects\Python Projects\punge\default\mp3\Kanye West - FamousLq2TmRzg19k.mp3",
    ));
    println!("x={}", x);
}
