use crate::db::fetch;
use crate::player::player_cache::{fetch_cache, Cache};
use crate::types::PungeMusicObject;
use rand;
use rand::seq::SliceRandom;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};

pub struct MusicPlayer {
    pub list: Vec<PungeMusicObject>,
    pub sink: rodio::Sink,
    pub count: isize,
    pub shuffle: bool,
    pub to_play: bool,
    pub stream: rodio::OutputStream,
    pub current_object: PungeMusicObject, // represents the playing song. used in shuffle to get back to it
}

unsafe impl Send for MusicPlayer {}
unsafe impl Sync for MusicPlayer {}

impl MusicPlayer {
    pub fn new(mut list: Vec<PungeMusicObject>) -> MusicPlayer {
        // Music player and the song that will be used to update the gui
        let cache: Cache = fetch_cache();
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.set_volume(cache.volume);
        if cache.shuffle {
            let mut rng = rand::thread_rng();
            list.shuffle(&mut rng);
        }

        let count = list
            .iter()
            .position(|r| r.clone().uniqueid == cache.song_id)
            .unwrap_or(0);
        let current_object = list[count as usize].clone();
        // list should inherite from cache at some point. not worried now tho
        MusicPlayer {
            list,
            sink,
            count: count as isize,
            shuffle: cache.shuffle,
            to_play: false,
            stream,
            current_object,
        }
    }

    fn fetch_and_update_playlist(&mut self, playlist_name: String) {
        let playlist_uuid = fetch::get_uuid_from_name(playlist_name);
        let new =
            fetch::get_all_from_playlist(playlist_uuid.as_str()).expect("playlist uuid not found:");
        self.list = new;
    }

    fn play_from_time(&mut self, time: usize) {
        // used when playing from the scrubbing bar
        self.sink.stop(); // is this required? likely
        self.sink.append(read_from_time(
            self.list[self.count as usize].savelocationmp3.clone(),
            time,
        ));
        // self.play_loop()
    }
}

pub fn read_file_from_beginning(file: String) -> Decoder<BufReader<File>> {
    println!("file: {}", &file);
    // we should overhaul this at some point to be a method associated with the app. when there is a file that doesn't exist,
    // we can send it to some related "missing" vector. this can be written to json when program closes? or when found?
    let reader = BufReader::new(File::open(file).unwrap());
    let decoder = Decoder::new(reader).unwrap();
    decoder
}

pub fn read_from_time(file: String, time: usize) -> Decoder<BufReader<File>> {
    let mut reader = BufReader::new(File::open(file).unwrap());
    let sample_rate = 44100; // yt songs are always 44100
    let position = sample_rate * time;
    reader.seek(SeekFrom::Start(position as u64)).unwrap();
    let decoder: Decoder<BufReader<File>> = Decoder::new(reader).unwrap();
    decoder
}
