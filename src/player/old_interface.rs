use rodio::{OutputStream, Sink, Decoder};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use std::sync::{mpsc::Receiver, mpsc};
use crate::playliststructs::PungeMusicObject;
use crate::fetch;


pub enum Command {
    Play,
    Stop,
    ChangeSong(usize), // play this song at this index in the list. also, do we need this as &str for thread safety?
    NewVolume(usize),  // change volume to this amount (processed beforehand I think)
    SkipToSeconds(usize),  // intends to play current song from this time (bcs only active song can be target of this operation)
    SkipForwards,
    SkipBackwards,
    StaticVolumeUp,  // used for binds to increase volume by x amount
    StaticVolumeDown,
    ToggleShuffle,  // will either shuffle or unshuffle the playlist
    GoToAlbum,  // not implemented yet. will be used as change the surrounding playlist to the album the song is from
    ChangePlaylist(String),  // change the current playlist to the one specified here
}

pub struct MusicPlayer {
    pub list: Vec<PungeMusicObject>,
    pub sink: rodio::Sink,
    pub count: isize,
    pub listener: mpsc::Receiver<Command>,
    pub shuffle: bool
}

impl MusicPlayer {
    pub fn new(list: Vec<PungeMusicObject>, listener: Receiver<Command>) -> MusicPlayer {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    // here, we leak `OutputStream` to make the audio not stop (if bevy does it, so can i :D )
    std::mem::forget(stream);
    let sink = Sink::try_new(&stream_handle).unwrap();
     MusicPlayer {
         list,
         sink,
         count: 0,
         listener,
         shuffle: false  // this should be derived from the json that logs this data
     }
    }


    fn fetch_and_update_playlist(&mut self, playlist_name: String) {
        let playlist_uuid = fetch::get_uuid_from_name(playlist_name);
        let new = fetch::get_all_from_playlist(playlist_uuid.as_str()).expect("playlist uuid not found:");
        self.list = new;
    }

    fn play_from_time(&mut self, time: usize) {
        // used when playing from the scrubbing bar
        self.sink.stop(); // is this required? likely
        self.sink.append(read_from_time(self.list[self.count as usize].savelocationmp3.clone(), time));
        self.play_loop()
    }
    // maybe revise at some point i dont think there needs to be so many receiver checks .. fine for now
   pub fn play_loop(&mut self) {
        loop {
            self.process_command(); // process the commands and change needed data
            if self.count < 0 {
                // if the count is in the negatives, we can set the val as if it was python. my_list[-3]
               // println!("count here:? {} | {}", self.count, self.list.len());
                self.count = (self.list.len() as isize + self.count) as isize;  // was '-' before
            }
            if self.count >= (self.list.len() as isize) {
                self.count = 0
            }
            if self.sink.empty() {
                self.sink.append(read_file_from_beginning(self.list[self.count as usize].savelocationmp3.clone()));
            }
            self.sink.play();
            while !self.sink.empty() {
                self.process_command(); // process inside of the nested loop. is this needed? im not sure
            if self.sink.is_paused() {
                // println!("is paused: stopping & breaking | incremnting count from {} to {}", self.count, self.count + 1);
                 // self.count += 1; why were we pausing here?
                   break
                }
                else {
                std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
            if self.sink.is_paused() {
                // so if pause is clicked, will  very quickly break from both loops
                break
            }
            else { // this should be the case where the song just plays out. so we increment by 1
                // this gets hit when skipping... just like old punge. maybe should be fixed at some point
                // would also require a rewrite of skipping backwards and forwards (which is good)
                self.count += 1;
            }

        }
    }

    pub fn process_command(&mut self) {
        match self.listener.try_recv() {
            Ok(Command::Play) => {
                self.play_loop();
            }
            Ok(Command::Stop) => {
                self.sink.pause();
            }
            Ok(Command::ChangeSong(e)) => {
                self.count = e as isize;
                self.play_loop();
            }
            Ok(Command::NewVolume(e)) => {
                self.sink.set_volume(e as f32)  // zero clue what this will do lol. careful when testing
            }
            Ok(Command::SkipToSeconds(sec)) => {
                self.play_from_time(sec)
            }
            Ok(Command::SkipForwards) => {
                // self.count += 1;  // this also may cause problems in the future for this bit here lol
                self.sink.stop();
                self.play_loop();
            }
            Ok(Command::SkipBackwards) => {
                self.count = if self.count == 0 {  // counteracts subtract overflows (the loop one may be obsolete now?)
                    self.list.len() as isize -2  // we can set this equal to list[-2]
                }
                    else if self.count == 1 {
                        self.list.len() as isize -1 // this will get set to 0
                    }
                    else {
                        self.count - 2
                    };
                self.sink.stop();
                self.play_loop();
            }
            Ok(Command::StaticVolumeUp) => {
                self.sink.set_volume(self.sink.volume() + 5.0)  // no clue how 'much' this is
            }
            Ok(Command::StaticVolumeDown) => {
                self.sink.set_volume(self.sink.volume() - 5.0)
            }
            Ok(Command::ToggleShuffle) => {
                if self.shuffle {
                    // ill do this shortly
                    // rand::seq__SliceRandom
                    // my_list.shuffle(&mut rand::thread::rng());
                }
                else {

                }
            }
            Ok(Command::GoToAlbum) => {
                // unimplemented
            }
            Ok(Command::ChangePlaylist(playlist)) => {
                // uninplemented
            }
            Err(e) => {
               // this is hit when no updates are sent. we can do nothing here

            }

        };
    }


}

fn read_file_from_beginning(file: String) -> Decoder<BufReader<File>> {
    let reader = BufReader::new(File::open(file).unwrap());
    let decoder = Decoder::new(reader).unwrap();
    decoder

}

fn read_from_time(file: String, time: usize) -> Decoder<BufReader<File>> {
    let mut reader = BufReader::new(File::open(file).unwrap());
    let sample_rate = 44100;  // yt songs are always 44100
    let position = sample_rate * time;
    reader.seek(SeekFrom::Start(position as u64)).unwrap();
    let decoder: Decoder<BufReader<File>> = Decoder::new(reader).unwrap();
    decoder
}