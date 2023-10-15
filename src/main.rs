extern crate core;

mod playliststructs;

mod db;
mod gui;
mod player;
mod utils;

fn main() {
    gui::start::begin().unwrap();
    //     println!(
    //         "{:?}",
    //         player::sort::get_values_from_db("main".to_string(), "Dream House".to_string())
    //     );
    // db::utilities::convert();
}
