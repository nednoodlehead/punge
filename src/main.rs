extern crate core;

mod playliststructs;

mod db;
mod gui;
mod player;
mod utils;

fn main() {
    gui::start::begin().unwrap();
}
