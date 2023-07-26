use rodio::{OutputStream, Sink, Decoder};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::sync::{mpsc::Receiver, mpsc};
mod playliststructs;

mod db;
mod gui;
mod player;


fn main() {
    gui::start::begin().unwrap();
}

