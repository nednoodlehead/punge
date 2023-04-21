// purpose of this file is to download youtube videos and name them appropriately. it calls
// various functions from "decide_youtube.rs" to determine the title, author and album


use rustube::Video;
use std::fs;
use pyo3::prelude::*;
use std::process::Command;

// #[path = "./decide_youtube.rs"]
// mod decide;
// use decide::entry;

use crate::playliststructs::{Playlist, PungeMusicObject};
mod playlist;

mod decide_youtube;
use decide_youtube::{begin_single, begin_playlist};


// this is the function exposed to the rest of the app. It takes in the youtube link
pub fn intro(link: &str) {
    // need to check if the file exists
    if link.contains("list=") {
        let vid = playlist_parse(link);
        begin_playlist(vid)
    }
    else {
        let vid = single_parse(link);
        let (artist,title,album) = begin_single(vid);
        println!("{} | {} | {}", artist, title, album)
    };


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

pub fn opus_to_mp3(file: &str, mut name: String) {
    /// we are assuming that the order is:
    /// download video
    /// convert to mp3
    /// rename / chop
    /// so we should have the original name of the file from here. name should include
    let mut directory_prefix: String = String::from("f:/"); // this should come from the json file that holds the directory of where mp3s and thumbnails are kept
    if !directory_prefix.ends_with("\\") || !directory_prefix.ends_with("/") {
        directory_prefix = directory_prefix + "/"
    }
    // if there is no .mp3 on the end of the filename, we need to add it
    if !name.to_lowercase().ends_with(".mp3") {
        name.to_owned();
        name += ".mp3";
    }
    let new_path_complete = directory_prefix + name.as_str();
    // call ffmpeg to turn the old webm file into an mp3 file
    let x = Command::new("ffmpeg.exe").args(&["-i", file, "-vn", "-c:a", "libmp3lame", "-b:a", "192k",
        new_path_complete.as_str()]).output();
    println!("ffmpeg status: {:?}", x.unwrap());
}





