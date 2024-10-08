
use crate::player::player_cache::{fetch_cache, Cache};
use crate::types::PungeMusicObject;
use rand;
use rand::seq::SliceRandom;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use chrono::Local;

pub struct MusicPlayer {
    pub list: Vec<PungeMusicObject>,
    pub playlist: String,
    pub sink: rodio::Sink,
    pub count: usize,
    pub shuffle: bool,
    pub to_play: bool,
    // we still need to hold it (im pretty sure), we just dont read it.
    pub _stream: rodio::OutputStream,
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
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
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
        let current_object = if list.is_empty() {
            PungeMusicObject {
                title: "No songs loaded".to_string(),
                author: "Download from the 'Download' tab".to_string(),
                album: "You'll probs have to restart".to_string(),
                features: "none".to_string(),
                length: 10,
                savelocationmp3: "none".to_string(),
                savelocationjpg: "none".to_string(),
                
            datedownloaded: Local::now().date_naive(),
            lastlistenedto: Local::now().date_naive(),
            ischild: true,
            uniqueid: "empty".to_string(),
            plays: 0,
            weight: 0,
            threshold: 1,
            order: 0
            }            
        }
        else {
            list[count].clone()
        };
        // list should inherite from cache at some point. not worried now tho
        MusicPlayer {
            list,
            playlist: cache.playlist_id,
            sink,
            count,
            shuffle: cache.shuffle,
            to_play: false,
            _stream,
            current_object,
        }
    }
}

pub fn read_file_from_beginning(file: &str) -> Decoder<BufReader<File>> {
    // we should overhaul this at some point to be a method associated with the app. when there is a file that doesn't exist,
    // we can send it to some related "missing" vector. this can be written to json when program closes? or when found?
    let reader = BufReader::new(File::open(file).unwrap());

    Decoder::new(reader).unwrap()
}

