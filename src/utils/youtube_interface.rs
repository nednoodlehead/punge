// purpose of this file is to download youtube videos and name them appropriately. it calls
// various functions from "decide_youtube.rs" to determine the title, author and album

use pyo3::prelude::*;
use rustube::Video;
use std::fs;
use std::process::Command;
use rusqlite;
use rusqlite::params;

// #[path = "./decide_youtube.rs"]
// mod decide;
// use decide::entry;

use crate::playliststructs::{Playlist, PungeMusicObject};
mod playlist;

mod decide_youtube;
use decide_youtube::{begin_playlist, begin_single};

// this is the function exposed to the rest of the app. It takes in the youtube link
pub fn intro(link: &str) {
    // need to check if the file exists
    if link.contains("list=") {
        let vid = playlist_parse(link);
        begin_playlist(vid)
    } else {
        let vid = single_parse(link);
        begin_single(vid);
    };
}

pub fn check_if_exists(uniqueid: String) {
    // checks if the given unique id is found inside the main table. aka: has it been downloaded?
    let conn = rusqlite::Connection::open("main.db").unwrap();
    let res: String = conn.query_row("SELECT EXISTS(SELECT 1 FROM main WHERE uniqueid=?)", params![uniqueid], |row| row.get(0))
        .unwrap();
    println!("res: {}", res)


}

fn playlist_parse(playlist_link: &str) -> Playlist {
    let playlist: Playlist = playlist::get_playlist(playlist_link);
    println!("{:?}", playlist);
    playlist
}

fn single_parse(single_link: &str) -> rustube::blocking::video::Video {
    // create a url type, used to create the video type
    let url = rustube::url::Url::parse(single_link).unwrap();

    let vid = rustube::blocking::Video::from_url(&url).unwrap();
    let info = vid.video_details();
    vid
}

