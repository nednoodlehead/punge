use crate::db::fetch;
use crate::gui::messages::AppEvent;
use crate::gui::messages::{Context, ProgramCommands, PungeCommand};
use crate::gui::start::App;
use crate::player::interface::{self};
use crate::player::interface::{read_file_from_beginning, read_from_time};
use crate::types::{Config, MusicData, PungeMusicObject, ShuffleType};
use arc_swap::ArcSwap;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

use global_hotkey::{GlobalHotKeyEvent, HotKeyState};

use iced::futures::sink::SinkExt;
use iced::subscription::Subscription;
use rand::seq::SliceRandom;
use rand::{self, Rng};
use std::sync::Arc;
use std::time::Instant;
use tokio::{self}; // for benchmarking the skip function

// makes idling a bit more interesting. Could pull these from json one day...
const IDLE_STRINGS: &[&str] = &[
    "Chillin in Punge",
    "Listening to nothin..",
    "Listening to the birds chirp",
    "Idle in Punge menus",
    "Probably afk lol",
    "Currently touching grass..",
    "Busy improving Punge",
    "Taking the dog for a walk",
    "Listening to silence",
    "Music stopped, locked in 😈",
    "Honestly, probably getting food.",
    "Piggin out",
    "Writing more stupid quips",
    "Doing a war attack",
    "Right ear: Silence. Left ear: tinnitus",
];
impl App {
    // difference between this database subscription is that no sender and receiver is needed, instead we check the status of self.current_obj every 20 seconds or so and do some calculations for inserting into db
    // now the question you may have is, "ok, i see how this can work for weight, but how for plays?", because the weight can be adjusted maybe += 1 for each 20 seconds its listened
    // well, to answer the question of "how do we calculate plays", is we can divide the video up by the increment value, and if it reaches that value, add +1 play
    // also, in the other download function , we divide the len by 25 to see how many times it fits,  we will have the db check every 15 seconds,
    pub fn database_subscription(
        &self,
        obj: Arc<ArcSwap<MusicData>>,
    ) -> Subscription<ProgramCommands> {
        iced::subscription::channel(11, 32, |mut _sender| async move {
            async_std::task::sleep(std::time::Duration::from_secs(4)).await; // give the id time to init properly, no real rush to have the subscription start right away anyways...
            let mut id = obj.load().song_id.clone(); // hopfully initialized in time
            let mut cycle = 0;
            loop {
                if !obj.load().is_playing {
                    loop {
                        async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                        if obj.load().is_playing {
                            break;
                        }
                    }
                }
                async_std::task::sleep(std::time::Duration::from_secs(10)).await;
                if id == obj.load().song_id {
                    cycle += 1;
                    crate::db::metadata::add_one_weight(obj.load().song_id.clone()).unwrap();
                    if cycle == obj.load().threshold {
                        // so doing it this way gets rid of the need to hold onto the last id, since midway through (~2/3rd way) +1 play will occur
                        println!("added one to play");
                        crate::db::metadata::add_one_play(obj.load().song_id.clone()).unwrap();
                    }
                } else {
                    //song has changed, was the threshold met?
                    id = obj.load().song_id.clone();
                    cycle = 0;
                }
            }
        })
    }

    pub fn hotkey_loop(&self, config: Arc<ArcSwap<Config>>) -> Subscription<ProgramCommands> {
        iced::subscription::channel(5, 32, |mut sender| async move {
            // so can we have a hashmap, that can be updated, and the loop here will do a hashmap lookup
            // for those types and find the associated command, and send it?
            // so map {
            //     9181: SkipForwards,
            // }
            loop {
                // config needs to be refreshed each loop, cause if it isn't, it won't get updates for new binds
                // wish this could be done better?
                // println!("da config: {:?}", &config.load());
                match GlobalHotKeyEvent::receiver().try_recv() {
                    Ok(hotkey) => {
                        if hotkey.state == HotKeyState::Pressed {
                            let open_con = config.load();
                            // only do something when it is pressed
                            // handle global keybinds
                            let id = hotkey.id;
                            // if the keybind is registered!
                            if open_con.keybinds.contains_key(&id) {
                                sender
                                    .send(open_con.keybinds[&id].command.clone())
                                    .await
                                    .unwrap();
                            }
                        }
                        // send the keybind back to main gui
                    }
                    Err(_e) => {
                        // erm, ignore
                    }
                }
                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
                // required for the stuff to work
            }
        })
    }

    pub fn close_app_sub(&self) -> Subscription<ProgramCommands> {
        // bro they took my events_with
        iced::event::listen_with(handle_app_events)
        // nvmd i got it back
    }

    pub fn music_loop(&self, config: Arc<ArcSwap<Config>>) -> Subscription<ProgramCommands> {
        iced::subscription::channel(0, 32, |mut sender| async move {
            // sender to give to the gui, and the receiver is used here to listen for clicking of buttons
            let (gui_send, mut gui_rec) = tokio::sync::mpsc::unbounded_channel();
            sender
                .send(ProgramCommands::UpdateSender(Some(gui_send)))
                .await
                .unwrap(); // send the sender to the gui !!
            let items: Vec<PungeMusicObject> = fetch::get_all_main().unwrap();
            // maybe here  we need to get index of last song that was on?
            // send the data to the program
            let mut music_obj = interface::MusicPlayer::new(items);
            sender
                .send(ProgramCommands::NewData(MusicData {
                    title: music_obj.current_object.title.clone(),
                    author: music_obj.current_object.author.clone(),
                    album: music_obj.current_object.album.clone(),
                    song_id: music_obj.current_object.uniqueid.clone(),
                    previous_id: None, // doesnt matter unless we are on skip_forward
                    volume: music_obj.sink.volume(),
                    is_playing: false,
                    shuffle: music_obj.shuffle,
                    playlist: music_obj.playlist.clone(),
                    threshold: music_obj.current_object.threshold,
                    context: Context::Default,
                    length: music_obj.current_object.length,
                }))
                .await
                .unwrap();

            // main music loop!
            println!("starting main loop");
            println!("Struct from start: {:?}", &music_obj);
            loop {
                match gui_rec.try_recv() {
                    Ok(cmd) => match cmd {
                        PungeCommand::PlayOrPause => {
                            if music_obj.sink.empty() {
                                let song = interface::read_file_from_beginning(
                                    music_obj.list[music_obj.count].savelocationmp3.clone(),
                                );
                                music_obj.sink.append(song);
                            }
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            println!("playing here... {:?}", &music_obj);
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.album.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: None, // i dont think we can know this here?
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: music_obj.playlist.clone(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::PlayPause,
                                    length: music_obj.current_object.length,
                                }))
                                .await
                                .unwrap();
                        }
                        PungeCommand::SkipForwards => {
                            // so i guess the answer is doing .stop()? not .clear(). ig cause .stop() also clears the queue
                            music_obj.sink.stop();
                            println!("skip forards, top!! {:?}", &music_obj);
                            let old_id = music_obj.current_object.uniqueid.clone();
                            music_obj.count =
                                change_count(true, music_obj.count, music_obj.list.len());
                            music_obj.current_object = music_obj.list[music_obj.count].clone();

                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count].savelocationmp3.clone(),
                            ));
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.album.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: Some(old_id),
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: music_obj.playlist.clone(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::SkippedForward,
                                    length: music_obj.current_object.length,
                                }))
                                .await
                                .unwrap();
                        }
                        PungeCommand::SkipBackwards => {
                            music_obj.sink.stop();
                            music_obj.count =
                                change_count(false, music_obj.count, music_obj.list.len());
                            music_obj.current_object = music_obj.list[music_obj.count].clone();
                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count].savelocationmp3.clone(),
                            ));
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.album.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: None,
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: music_obj.playlist.clone(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::SkippedBackwards,
                                    length: music_obj.current_object.length,
                                }))
                                .await
                                .unwrap();
                        }
                        PungeCommand::NewVolume(val) => {
                            music_obj.sink.set_volume((val as f32) / 80.0)
                        }
                        PungeCommand::ChangeSong(uuid) => {
                            let index = music_obj
                                .list
                                .iter()
                                .position(|r| r.clone().uniqueid == uuid)
                                .unwrap();

                            music_obj.sink.stop();
                            music_obj.count = index;
                            music_obj.current_object = music_obj.list[music_obj.count].clone();
                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count].savelocationmp3.clone(),
                            ));
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.album.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: None,
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: music_obj.playlist.clone(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::Seeked,
                                    length: music_obj.current_object.length,
                                }))
                                .await
                                .unwrap();
                        }
                        PungeCommand::GoToAlbum => {
                            println!("going 2 album!")
                        }
                        PungeCommand::SkipToSeconds(val) => {
                            println!("skipping to seconds. paused?");
                            music_obj.sink.append(read_from_time(
                                music_obj.list[music_obj.count].savelocationmp3.clone(),
                                val,
                            ));
                            // no play, since we are paused
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.album.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: None,
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: music_obj.playlist.clone(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::Seeked,
                                    length: music_obj.current_object.length,
                                }))
                                .await
                                .unwrap();
                        }
                        PungeCommand::ToggleShuffle => {
                            println!("DEBUG HERE: {}", &music_obj.list[music_obj.count].title);
                            if music_obj.shuffle {
                                if music_obj.playlist == "main" {
                                    music_obj.list = fetch::get_all_main().unwrap();
                                } else {
                                    music_obj.list =
                                        fetch::get_all_from_playlist(&music_obj.playlist).unwrap();
                                }
                                // it is shuffled, lets re-order
                                let index = music_obj // todo ok, need to put back in order
                                    .list
                                    .iter()
                                    .position(|r| {
                                        r.clone().uniqueid == music_obj.current_object.uniqueid
                                    })
                                    .unwrap();
                                println!("at inddex: {}", index);
                                music_obj.count = index;
                                music_obj.shuffle = false;
                            } else {
                                // now, we match from the config
                                music_obj.list = match config.load().shuffle_type {
                                    ShuffleType::Regular => {
                                        crate::player::sort::regular_shuffle(music_obj.list)
                                    }
                                    ShuffleType::WeightBias => {
                                        crate::player::sort::shuffle_weight_bias(music_obj.list)
                                    }
                                    ShuffleType::TrueRandom => {
                                        crate::player::sort::true_random_shuffle(music_obj.list)
                                    }
                                };
                                music_obj.shuffle = true;
                                // ok this seems to fix #33, but why does the non-paused version not need this?
                                // i probably would've noticed something by now...
                                let index = music_obj
                                    .list
                                    .iter()
                                    .position(|r| {
                                        r.clone().uniqueid == music_obj.current_object.uniqueid
                                    })
                                    .unwrap();
                                music_obj.count = index;
                            }
                        }
                        PungeCommand::ChangePlaylist(name) => {
                            println!("changing the playlist! above");
                            if name == "main" {
                                music_obj.list = fetch::get_all_main().unwrap();
                            } else {
                                println!("getting all from {}", &name);
                                music_obj.list = fetch::get_all_from_playlist(&name).unwrap();
                            }
                            music_obj.playlist = name;
                        }
                    },
                    _ => {
                        // what gets hit when nothing happens
                    }
                }
                if music_obj.to_play {
                    // if we are playing, we want to loop and keep playing !!
                    loop {
                        // i think most of the count checks are depciated
                        println!("inside our palying loop!");
                        // process commands (maybe turn it into a function i guess?, would sort of suck to copy and paste to make work)
                        if music_obj.count == 0 {
                            music_obj.count += music_obj.list.len();
                        }
                        if music_obj.count >= (music_obj.list.len()) {
                            music_obj.count = 0;
                        }
                        if music_obj.sink.empty() {
                            println!("default appending!");
                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count].savelocationmp3.clone(),
                            ));
                        }
                        println!("playing, in theory");
                        music_obj.sink.play();
                        while !music_obj.sink.is_paused() {
                            // process again !?
                            match gui_rec.try_recv() {
                                Ok(cmd) => {
                                    match cmd {
                                        PungeCommand::PlayOrPause => {
                                            // so the logic here, is that the only command issued here is pause, play cannot occur from this location. (cause the loop ends and we are in the top area)
                                            println!("stooping here! (bottom)");
                                            sender
                                                .send(ProgramCommands::NewData(MusicData {
                                                    title: music_obj.current_object.title.clone(),
                                                    author: music_obj.current_object.author.clone(),
                                                    album: music_obj.current_object.album.clone(),
                                                    song_id: music_obj
                                                        .current_object
                                                        .uniqueid
                                                        .clone(),
                                                    previous_id: None, // not known.
                                                    volume: music_obj.sink.volume(),
                                                    is_playing: false, // we can only pause...
                                                    shuffle: music_obj.shuffle,
                                                    playlist: music_obj.playlist.clone(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::SkippedForward,
                                                    length: music_obj.current_object.length,
                                                }))
                                                .await
                                                .unwrap();
                                            music_obj.sink.pause();
                                            music_obj.to_play = false
                                        }
                                        PungeCommand::SkipForwards => {
                                            let start = Instant::now();
                                            println!("skippin forrards");
                                            println!("obj skip forwards: {:?}", &music_obj);
                                            music_obj.sink.stop(); // why was this not here before and how did it even work !?
                                            let old_id = music_obj.current_object.uniqueid.clone();
                                            music_obj.count = change_count(
                                                true,
                                                music_obj.count,
                                                music_obj.list.len(),
                                            );
                                            music_obj.current_object =
                                                music_obj.list[music_obj.count].clone();
                                            music_obj.sink.append(read_file_from_beginning(
                                                music_obj.list[music_obj.count]
                                                    .savelocationmp3
                                                    .clone(),
                                            ));
                                            music_obj.to_play = true;
                                            music_obj.sink.play();
                                            sender
                                                .send(ProgramCommands::NewData(MusicData {
                                                    title: music_obj.current_object.title.clone(),
                                                    author: music_obj.current_object.author.clone(),
                                                    album: music_obj.current_object.album.clone(),
                                                    song_id: music_obj
                                                        .current_object
                                                        .uniqueid
                                                        .clone(),
                                                    previous_id: Some(old_id),
                                                    volume: music_obj.sink.volume(),
                                                    is_playing: true,
                                                    shuffle: music_obj.shuffle,
                                                    playlist: music_obj.playlist.clone(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::SkippedForward,
                                                    length: music_obj.current_object.length,
                                                }))
                                                .await
                                                .unwrap();
                                            let end = Instant::now();
                                            println!(
                                                "time elapsed (lower) {:?}",
                                                end.duration_since(start)
                                            )
                                        }
                                        PungeCommand::SkipBackwards => {
                                            // music_obj.count -= 1; // do check for smaller than music_obj.len()?
                                            music_obj.count = change_count(
                                                false,
                                                music_obj.count,
                                                music_obj.list.len(),
                                            );
                                            music_obj.current_object =
                                                music_obj.list[music_obj.count].clone();
                                            if !music_obj.sink.is_paused() {
                                                // so this if stmt was on the upper match stmt, but kept causing problems with skipping and clearing the sink (even tho
                                                // the if occurs before the sink.append() ). so it only is down here, and seems to work just fine
                                                music_obj.sink.stop();
                                                music_obj.sink.clear()
                                            }
                                            music_obj.sink.append(read_file_from_beginning(
                                                music_obj.list[music_obj.count]
                                                    .savelocationmp3
                                                    .clone(),
                                            ));
                                            music_obj.to_play = true;
                                            music_obj.sink.play();
                                            sender
                                                .send(ProgramCommands::NewData(MusicData {
                                                    title: music_obj.current_object.title.clone(),
                                                    author: music_obj.current_object.author.clone(),
                                                    album: music_obj.current_object.album.clone(),
                                                    song_id: music_obj
                                                        .current_object
                                                        .uniqueid
                                                        .clone(),
                                                    previous_id: None,
                                                    volume: music_obj.sink.volume(),
                                                    is_playing: true,
                                                    shuffle: music_obj.shuffle,
                                                    playlist: music_obj.playlist.clone(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::SkippedBackwards,
                                                    length: music_obj.current_object.length,
                                                }))
                                                .await
                                                .unwrap();
                                        }
                                        PungeCommand::NewVolume(val) => {
                                            music_obj.sink.set_volume((val as f32) / 80.0)
                                        }
                                        PungeCommand::SkipToSeconds(val) => {
                                            music_obj.sink.stop();
                                            music_obj.sink.append(read_from_time(
                                                music_obj.list[music_obj.count]
                                                    .savelocationmp3
                                                    .clone(),
                                                val,
                                            ));
                                            // play :D we are inside the playing loop
                                            music_obj.to_play = true;
                                            music_obj.sink.play();
                                            sender
                                                .send(ProgramCommands::NewData(MusicData {
                                                    title: music_obj.current_object.title.clone(),
                                                    author: music_obj.current_object.author.clone(),
                                                    album: music_obj.current_object.album.clone(),
                                                    song_id: music_obj
                                                        .current_object
                                                        .uniqueid
                                                        .clone(),
                                                    previous_id: None,
                                                    volume: music_obj.sink.volume(),
                                                    is_playing: true,
                                                    shuffle: music_obj.shuffle,
                                                    playlist: music_obj.playlist.clone(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::Seeked,
                                                    length: music_obj.current_object.length,
                                                }))
                                                .await
                                                .unwrap();
                                        }
                                        PungeCommand::ToggleShuffle => {
                                            if music_obj.shuffle {
                                                if music_obj.playlist == "main" {
                                                    music_obj.list = fetch::get_all_main().unwrap();
                                                } else {
                                                    music_obj.list = fetch::get_all_from_playlist(
                                                        &music_obj.playlist,
                                                    )
                                                    .unwrap();
                                                }
                                                let index = music_obj
                                                    .list
                                                    .iter()
                                                    .position(|r| {
                                                        r.clone().uniqueid
                                                            == music_obj.current_object.uniqueid
                                                    })
                                                    .unwrap();
                                                println!("indexing: {}", index);
                                                music_obj.count = index;
                                                music_obj.shuffle = false;
                                            } else {
                                                music_obj.list = match config.load().shuffle_type {
                                                    ShuffleType::Regular => {
                                                        crate::player::sort::regular_shuffle(
                                                            music_obj.list,
                                                        )
                                                    }
                                                    ShuffleType::WeightBias => {
                                                        crate::player::sort::shuffle_weight_bias(
                                                            music_obj.list,
                                                        )
                                                    }
                                                    ShuffleType::TrueRandom => {
                                                        crate::player::sort::true_random_shuffle(
                                                            music_obj.list,
                                                        )
                                                    }
                                                };
                                                music_obj.shuffle = true;
                                            }
                                        }
                                        PungeCommand::ChangePlaylist(name) => {
                                            println!("changin the playlist below {}", &name);
                                            if name == "main" {
                                                music_obj.list = fetch::get_all_main().unwrap();
                                            } else {
                                                println!("getting all from {}", &name);
                                                music_obj.list =
                                                    fetch::get_all_from_playlist(&name).unwrap();
                                            }
                                            music_obj.playlist = name;
                                            println!("length below: {}", music_obj.list.len())
                                        }
                                        PungeCommand::ChangeSong(uuid) => {
                                            println!(
                                                "CURRENT PLAYLIST: {} and {}",
                                                music_obj.playlist,
                                                music_obj.list.len()
                                            );
                                            let index = music_obj
                                                .list
                                                .iter()
                                                .position(|r| r.clone().uniqueid == uuid)
                                                .unwrap();

                                            music_obj.count = index;
                                            music_obj.current_object =
                                                music_obj.list[music_obj.count].clone();
                                            if !music_obj.sink.is_paused() {
                                                // so this if stmt was on the upper match stmt, but kept causing problems with skipping and clearing the sink (even tho
                                                // the if occurs before the sink.append() ). so it only is down here, and seems to work just fine
                                                music_obj.sink.stop();
                                                music_obj.sink.clear()
                                            }
                                            music_obj.sink.append(read_file_from_beginning(
                                                music_obj.list[music_obj.count]
                                                    .savelocationmp3
                                                    .clone(),
                                            ));
                                            music_obj.to_play = true;
                                            music_obj.sink.play();
                                            sender
                                                .send(ProgramCommands::NewData(MusicData {
                                                    title: music_obj.current_object.title.clone(),
                                                    author: music_obj.current_object.author.clone(),
                                                    album: music_obj.current_object.album.clone(),
                                                    song_id: music_obj
                                                        .current_object
                                                        .uniqueid
                                                        .clone(),
                                                    previous_id: None,
                                                    volume: music_obj.sink.volume(),
                                                    is_playing: true,
                                                    shuffle: music_obj.shuffle,
                                                    playlist: music_obj.playlist.clone(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::Seeked,
                                                    length: music_obj.current_object.length,
                                                }))
                                                .await
                                                .unwrap();
                                        }
                                        _ => {
                                            println!("yeah, other stuff... {:?}", cmd)
                                        }
                                    }
                                }
                                _ => {
                                    // what gets hit when nothing happens
                                }
                            }
                            if music_obj.sink.is_paused() {
                                println!("is paused break!");
                                break;
                            } else if music_obj.sink.empty() {
                                println!("empty break!! ");
                                break;
                            } else {
                                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
                            }
                        }
                        if music_obj.sink.is_paused() {
                            break;
                        } else {
                            println!("default counter!");
                            music_obj.count =
                                change_count(true, music_obj.count, music_obj.list.len());
                            music_obj.current_object = music_obj.list[music_obj.count].clone();
                            // new info :P
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.album.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: None,
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: music_obj.playlist.clone(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::AutoPlay,
                                    length: music_obj.current_object.length,
                                }))
                                .await
                                .unwrap();
                        }
                    }
                }
                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
    }

    pub fn discord_loop(&self, obj: Arc<ArcSwap<MusicData>>) -> Subscription<ProgramCommands> {
        iced::subscription::channel(13, 32, |mut _sender| async move {
            let mut client = DiscordIpcClient::new("1219029975441608737").unwrap();
            let punge_img = activity::Assets::new().large_image("punge_icon_for_discord-02");
            client.connect().unwrap();
            loop {
                // every 5 seconds, update the song. maybe this will be changed at some point to include the
                if !obj.load().is_playing {
                    client
                        .set_activity(
                            activity::Activity::new()
                                .state(
                                    IDLE_STRINGS
                                        [rand::thread_rng().gen_range(0..IDLE_STRINGS.len())],
                                )
                                .assets(punge_img.clone()),
                        )
                        .unwrap();
                    loop {
                        // loop so the idle message doesn't change repeatedly...
                        if obj.load().is_playing {
                            break;
                        } else {
                            async_std::task::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    }
                } else {
                    let tmp = obj.load();
                    let (title, artist) = (tmp.title.clone(), tmp.author.clone());
                    client
                        .set_activity(
                            activity::Activity::new()
                                .state(title.as_str())
                                .details(artist.as_str())
                                .assets(punge_img.clone()),
                        )
                        .unwrap();
                }
                async_std::task::sleep(std::time::Duration::from_secs(5)).await;
            }
        })
    }
}

// handles app events, used for listening for the window close event (for now)
fn handle_app_events(event: iced::Event, _status: iced::event::Status) -> Option<ProgramCommands> {
    match &event {
        iced::Event::Window(_, iced::window::Event::CloseRequested) => {
            Some(ProgramCommands::InAppEvent(AppEvent::CloseRequested))
        }
        _ => None,
    }
}

pub fn change_count(incrementing: bool, count: usize, vec_len: usize) -> usize {
    // change the count without worrying about index errors
    let new_count: usize = if count == 0 && !incrementing {
        // if removing and count =0 (would make it -1)
        // going below the limit
        vec_len - 1
    } else if (count == (vec_len - 1)) && incrementing {
        0_usize // going above or equal the limit
    } else if incrementing {
        // all other cases!
        count + 1
    } else {
        count - 1
    };
    new_count
}
