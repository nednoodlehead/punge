extern crate core;
mod types;

mod db;
mod gui;
mod player;
mod utils;
mod yt;

fn main() {
    // db::create_db::create_table_defaults();
    // utils::time::legacy_old_time_to_new();
    gui::start::begin().unwrap();
    //     println!(
    //         "{:?}",
    //         player::sort::get_values_from_db("main".to_string(), "Dream House".to_string())
    //     );
    // db::utilities::convert();
}
