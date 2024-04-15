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
    pub playlist: String,
    pub sink: rodio::Sink,
    pub count: usize,
    pub shuffle: bool,
    pub to_play: bool,
    pub stream: rodio::OutputStream,
    pub current_object: PungeMusicObject, // represents the playing song. used in shuffle to get back to it
}
// ngl i aint know too much, but im pretty sure this could cause problems, but it makes the program work, so...
unsafe impl Send for MusicPlayer {}
unsafe impl Sync for MusicPlayer {}

impl std::fmt::Debug for MusicPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "list.len:{}\nplaylist:{}\nsink? {:?}\ncount={}\nshuffle={}\nto_play={}\ncurrent_obj={:?}\n",
            
        self.list.len(), self.playlist, "bruh", self.count, self.shuffle, self.to_play, self.current_object)
    }
}

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
        let current_object = list[count].clone();
        // list should inherite from cache at some point. not worried now tho
        MusicPlayer {
            list,
            playlist: cache.playlist,
            sink,
            count,
            shuffle: cache.shuffle,
            to_play: false,
            stream,
            current_object,
        }
    }
}

pub fn read_file_from_beginning(file: String) -> Decoder<BufReader<File>> {
    // we should overhaul this at some point to be a method associated with the app. when there is a file that doesn't exist,
    // we can send it to some related "missing" vector. this can be written to json when program closes? or when found?
    let reader = BufReader::new(File::open(file).unwrap());

    Decoder::new(reader).unwrap()
}

pub fn read_from_time(file: String, time: u32) -> Decoder<BufReader<File>> {
    let mut reader = BufReader::new(File::open(file).unwrap());
    let sample_rate = 48000; // 44100 or 192k?
    let position = sample_rate * time;
    reader.seek(SeekFrom::Start(position as u64)).unwrap();
    let decoder: Decoder<BufReader<File>> = Decoder::new(reader).unwrap();
    decoder
}
