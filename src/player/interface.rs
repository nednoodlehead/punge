use rodio::{OutputStream, Sink, Decoder, OutputStreamHandle};
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};
use iced_native::futures::channel::mpsc::{Receiver, Sender};
use crate::playliststructs;
use crate::gui;
use crate::db;
use crate::gui::messages::PungeCommand;
use crate::playliststructs::PungeMusicObject;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

pub static AUDIO_PLAYER: Mutex<Option<MusicPlayer>> = Mutex::new(None);

pub struct MusicPlayer {  // assembled once in ./gui/start.rs
    pub list: Vec<playliststructs::PungeMusicObject>,
    pub sink: rodio::Sink,
    pub count: AtomicUsize,
    pub shuffle: AtomicBool,
    pub stream: rodio::OutputStream,
    pub is_paused: AtomicBool,
    pub song_time: AtomicU64,  // how long the song has been playing for in ms
}
unsafe impl Send for MusicPlayer {}  // ok so these impls allow the musicplayer to be sent across threads
unsafe impl Sync for MusicPlayer {}  // idea comes from : https://github.com/Amazingkenneth/graduate/blob/192a29e1ab7a6090848a9725bc3091da2deeea57/src/audio.rs#L12
// this does cause some unexpected behaviour inside of the `sink` type, which is 'corrected' by using fields within the struct

impl MusicPlayer {
    pub fn new(stream_handle: OutputStreamHandle, stream: OutputStream, list: Vec<playliststructs::PungeMusicObject>) -> MusicPlayer {
    let sink = Sink::try_new(&stream_handle).unwrap();
     MusicPlayer {
         list,
         sink,
         count: AtomicUsize::new(1),
         shuffle: AtomicBool::new(false),  // this should be derived from the json that logs this data
         stream,
         is_paused: AtomicBool::new(true),
         song_time: AtomicU64::new(0)
     }
    }


    fn fetch_and_update_playlist(&mut self, playlist_name: String) {
        let playlist_uuid = db::fetch::get_uuid_from_name(playlist_name);
        let new = db::fetch::get_all_from_playlist(playlist_uuid.as_str()).expect("playlist uuid not found:");
        self.list = new;
    }

    fn play_from_time(&mut self, time: usize) {
        // used when playing from the scrubbing bar
        self.sink.stop(); // is this required? likely
        self.sink.append(read_from_time(self.list[self.count.load(Ordering::SeqCst) as usize].savelocationmp3.clone(), time));
        // self.play_loop()
    }
    // maybe revise at some point i dont think there needs to be so many receiver checks .. fine for now
   // pub fn play_loop(&mut self) {
   //      loop {
   //          self.process_command(); // process the commands and change needed data
   //          if self.count < 0 {
   //              // if the count is in the negatives, we can set the val as if it was python. my_list[-3]
   //             // println!("count here:? {} | {}", self.count, self.list.len());
   //              self.count = (self.list.len() as isize + self.count) as isize;  // was '-' before
   //          }
   //          if self.count >= (self.list.len() as isize) {
   //              self.count = 0
   //          }
   //          if self.sink.empty() {
   //              self.sink.append(read_file_from_beginning(self.list[self.count as usize].savelocationmp3.clone()));
   //          }
   //          self.sink.play();
   //          while !self.sink.empty() {
   //              self.process_command(); // process inside of the nested loop. is this needed? im not sure
   //          if self.sink.is_paused() {
   //              // println!("is paused: stopping & breaking | incremnting count from {} to {}", self.count, self.count + 1);
   //               // self.count += 1; why were we pausing here?
   //                 break
   //              }
   //              else {
   //              std::thread::sleep(std::time::Duration::from_millis(10));
   //              }
   //          }
   //          if self.sink.is_paused() {
   //              // so if pause is clicked, will  very quickly break from both loops
   //              break
   //          }
   //          else { // this should be the case where the song just plays out. so we increment by 1
   //              // this gets hit when skipping... just like old punge. maybe should be fixed at some point
   //              // would also require a rewrite of skipping backwards and forwards (which is good)
   //              self.count += 1;
   //          }
   //
   //      }
   //  }

    // pub fn process_command(&mut self) {
    //     match self.listener.try_next().unwrap() {
    //         Some(gui::messages::PungeCommand::Play) => {
    //             self.play_loop();
    //         }
    //         Some(gui::messages::PungeCommand::Stop) => {
    //             self.sink.pause();
    //         }
    //         Some(gui::messages::PungeCommand::ChangeSong(e)) => {
    //             self.count = e as isize;
    //             self.play_loop();
    //         }
    //         Some(gui::messages::PungeCommand::NewVolume(e)) => {
    //             self.sink.set_volume(e as f32)  // zero clue what this will do lol. careful when testing
    //         }
    //         Some(gui::messages::PungeCommand::SkipToSeconds(sec)) => {
    //             self.play_from_time(sec)
    //         }
    //         Some(gui::messages::PungeCommand::SkipForwards) => {
    //             // self.count += 1;  // this also may cause problems in the future for this bit here lol
    //             self.sink.stop();
    //             self.play_loop();
    //         }
    //         Some(gui::messages::PungeCommand::SkipBackwards) => {
    //             self.count = if self.count == 0 {  // counteracts subtract overflows (the loop one may be obsolete now?)
    //                 self.list.len() as isize -2  // we can set this equal to list[-2]
    //             }
    //                 else if self.count == 1 {
    //                     self.list.len() as isize -1 // this will get set to 0
    //                 }
    //                 else {
    //                     self.count - 2
    //                 };
    //             self.sink.stop();
    //             self.play_loop();
    //         }
    //         Some(gui::messages::PungeCommand::StaticVolumeUp) => {
    //             self.sink.set_volume(self.sink.volume() + 5.0)  // no clue how 'much' this is
    //         }
    //         Some(gui::messages::PungeCommand::StaticVolumeDown) => {
    //             self.sink.set_volume(self.sink.volume() - 5.0)
    //         }
    //         Some(gui::messages::PungeCommand::ToggleShuffle) => {
    //             if self.shuffle {
    //                 // ill do this shortly
    //                 // rand::seq__SliceRandom
    //                 // my_list.shuffle(&mut rand::thread::rng());
    //             }
    //             else {
    //
    //             }
    //         }
    //         Some(gui::messages::PungeCommand::GoToAlbum) => {
    //             // unimplemented
    //         }
    //         Some(gui::messages::PungeCommand::ChangePlaylist(playlist)) => {
    //             // uninplemented
    //         }
    //         None => {
    //            // this is hit when no updates are sent. we can do nothing here
    //
    //         }
    //         _ => {
    //             println!("unimplemented !!!")
    //         }
    //
    //     };
    // }


}





pub fn read_file_from_beginning(file: String) -> Decoder<BufReader<File>> {
    let reader = BufReader::new(File::open(file).unwrap());
    let decoder = Decoder::new(reader).unwrap();
    decoder
}

pub fn read_from_time(file: String, time: usize) -> Decoder<BufReader<File>> {
    let mut reader = BufReader::new(File::open(file).unwrap());
    let sample_rate = 44100;  // yt songs are always 44100
    let position = sample_rate * time;
    reader.seek(SeekFrom::Start(position as u64)).unwrap();
    let decoder: Decoder<BufReader<File>> = Decoder::new(reader).unwrap();
    decoder
}