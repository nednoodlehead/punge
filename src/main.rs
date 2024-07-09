extern crate core;
mod db;
mod gui;
mod player;
mod types;
mod utils;
mod yt;
use std::path::Path;

fn main() {
    // if the database is not found
    crate::utils::db::add_count_to_all_main();
    // if !Path::new("main.db").exists() {
    //     db::create_db::create_table_defaults().unwrap();
    // }
    // gui::start::begin().unwrap();
}
