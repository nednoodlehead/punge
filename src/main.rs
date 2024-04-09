extern crate core;
mod types;

mod db;
mod gui;
mod player;
mod utils;
mod yt;

fn main() {
    gui::start::begin().unwrap();
}
