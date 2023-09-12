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
    // utils::db_convert::convert_and_insert(String::from(
    //     r"F:\punge releases\punge_newest_2\Punge\MAINPLAYLIST.sqlite",
    // ));
        utils::db_convert::actually_download_old_songs();
}
