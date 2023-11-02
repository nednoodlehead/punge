use crate::db::fetch;
use crate::db::metadata::{add_one_play, add_one_weight, on_passive_play, on_seek, skipped_song};
use crate::gui::messages::AppEvent;
use crate::gui::messages::{Context, DatabaseMessages, ProgramCommands, PungeCommand};
use crate::gui::start::App;
use crate::player::interface::{self, MusicPlayer};
use crate::player::interface::{read_file_from_beginning, read_from_time};
use crate::playliststructs::MusicData;
use crate::playliststructs::PungeMusicObject;
use arc_swap::{ArcSwap, ArcSwapAny};
use async_std::task::sleep;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use iced::futures::channel::mpsc;
use iced::futures::sink::SinkExt;
use iced::subscription::{self, Subscription};
use rand::seq::SliceRandom;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::UnboundedSender;
use tokio::{self, runtime}; // for benchmarking the skip function
impl App {
    // requires a listener. this will be a tokio::UnboundedReceiver<PungeCommand>
    // does not need 2 way communication, as this subscription just listens and inserts into the database

    // difference between this database subscription is that no sender and receiver is needed, instead we check the status of self.current_obj every 20 seconds or so and do some calculations for inserting into db
    // now the question you may have is, "ok, i see how this can work for weight, but how for plays?", because the weight can be adjusted maybe += 1 for each 20 seconds its listened
    // well, to answer the question of "how do we calculate plays", is we can divide the video up by the increment value, and if it reaches that value, add +1 play
    // also, in the other download function , we divide the len by 25 to see how many times it fits,  we will have the db check every 15 seconds,
    pub fn database_sub(
        &self,
        mut receiver: tokio::sync::mpsc::UnboundedReceiver<MusicData>,
    ) -> Subscription<ProgramCommands> {
        iced::subscription::channel(8, 32, |mut sender| async move {
            println!("SENT TO MAIN ThrEAD");
            loop {
                match receiver.try_recv() {
                    Ok(t) => match t.context {
                        Context::Default => {
                            sender.send(ProgramCommands::NewData(t)).await.unwrap();
                            //as dasd
                        }
                        Context::PlayPause => {
                            sender.send(ProgramCommands::NewData(t)).await.unwrap();
                            // asd
                        }
                        Context::SkippedForward => {
                            // wrong songid, need one prior, adding
                            // skipped_song(t.previous_id.clone().unwrap()).unwrap();
                            sender.send(ProgramCommands::NewData(t)).await.unwrap();
                        }
                        Context::SkippedBackwards => {
                            sender.send(ProgramCommands::NewData(t)).await.unwrap();
                        }
                        Context::Seeked => {
                            on_seek(t.song_id.clone()).unwrap();
                            sender.send(ProgramCommands::NewData(t)).await.unwrap();
                            // db weight += 4 idk
                        }
                        Context::AutoPlay => {
                            on_passive_play(t.song_id.clone()).unwrap();
                            sender.send(ProgramCommands::NewData(t)).await.unwrap();
                            // db play += 1
                        }
                    },
                    Err(_e) => {
                        // ignore !!
                    }
                }

                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
    }

    // pub fn new_db_sub(
    //     &self,
    //     music_obj: Arc<ArcSwap<Arc<MusicData>>>,
    // ) -> iced::Subscription<ProgramCommands> {
    //     iced::subscription::channel(10, 32, |mut _sender| async move {
    //         let mut cycle = 0;
    //         let mut last_obj = **music_obj.load();
    //         let mut old_id: String = String::default();
    //         loop {
    //             if !last_obj.is_playing {
    //                 loop {
    //                     async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    //                     if last_obj.is_playing {
    //                         break;
    //                     }
    //                 }
    //             }
    //             async_std::task::sleep(std::time::Duration::from_secs(15)).await;
    //             if last_obj.song_id == old_id {
    //                 // add to weight
    //                 println!("Add to weight!!!");
    //                 cycle += 1; // add one to cycle, so we can keep track of
    //             } else {
    //                 // a new song is playing!
    //                 if cycle == last_obj.threshold {
    //                     old_id = last_obj.song_id.clone();
    //                     // threshold has been met for one to count towards playing. if we do >= instead, all above threshold will be added as a play
    //                     // so for example, if we listen to a song the entire way, that requirement may be met multiple times, but we did not listen multiple times...
    //                     // add one play, and weight or whatever from crate::db::metadata
    //                     println!("add one to metadata!!");
    //                 }
    //                 // then, bcs we are on a new song, reset the data
    //                 last_obj = **music_obj.load();
    //                 cycle = 0;
    //             }
    //         }
    //     })
    // }
    pub fn database_subscription(
        &self,
        obj: Arc<ArcSwap<Arc<MusicData>>>,
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
                    println!(
                        "add one to weight!! cycle: {} vs {}",
                        cycle,
                        obj.load().threshold
                    );
                    crate::db::metadata::add_one_weight(obj.load().song_id.clone()).unwrap();
                    if cycle == obj.load().threshold {
                        // so doing it this way gets rid of the need to hold onto the last id, since midway through (~2/3rd way) +1 play will occur
                        println!("added one to play");
                        crate::db::metadata::add_one_play(obj.load().song_id.clone()).unwrap();
                    }
                } else {
                    //song has changed, was the threshold met?
                    println!("song changed");
                    id = obj.load().song_id.clone();
                    cycle = 0;
                }
                println!("value inside sub: {:?}", **obj.load());
                println!("\n\n last song: {:?}", id);
            }
        })
    }

    pub fn hotkey_loop(&self) -> Subscription<ProgramCommands> {
        iced::subscription::channel(5, 32, |mut sender| async move {
            loop {
                match GlobalHotKeyEvent::receiver().try_recv() {
                    Ok(hotkey) => {
                        // handle global keybinds
                        println!("new keybind incming: {:?}", hotkey);
                        match hotkey {
                            GlobalHotKeyEvent { id: 4121890298 } => {
                                // right arrow
                                sender
                                    .send(ProgramCommands::Send(PungeCommand::SkipForwards))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 2037224482 } => {
                                // up arrow
                                sender
                                    .send(ProgramCommands::Send(PungeCommand::StaticVolumeUp))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 1912779161 } => {
                                // left arrow??
                                sender
                                    .send(ProgramCommands::Send(PungeCommand::SkipBackwards))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 4174001518 } => {
                                // down arrow!
                                sender
                                    .send(ProgramCommands::Send(PungeCommand::StaticVolumeDown))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 3520754938 } => {
                                // page down (shuffle)
                                sender
                                    .send(ProgramCommands::Send(PungeCommand::ToggleShuffle))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 3009842507 } => {
                                // end (pause)
                                sender
                                    .send(ProgramCommands::Send(PungeCommand::PlayOrPause))
                                    .await
                                    .unwrap()
                            }

                            _ => {
                                println!("anything else")
                            }
                        }
                    }
                    Err(e) => {
                        // erm, ignore
                    }
                }
                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
                // required for the stuff to work
            }
        })
    }

    pub fn close_app_sub(&self) -> Subscription<ProgramCommands> {
        iced::subscription::events_with(handle_app_events)
    }

    pub fn music_loop(&self) -> Subscription<ProgramCommands> {
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
                    playlist: "main".to_string(),
                    threshold: music_obj.current_object.threshold,
                    context: Context::Default,
                }))
                .await
                .unwrap();

            // main music loop!
            println!("starting main loop");
            loop {
                match gui_rec.try_recv() {
                    Ok(cmd) => match cmd {
                        PungeCommand::PlayOrPause => {
                            if music_obj.sink.empty() {
                                let song = interface::read_file_from_beginning(
                                    music_obj.list[music_obj.count as usize]
                                        .savelocationmp3
                                        .clone(),
                                );
                                music_obj.sink.append(song);
                            }
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            println!("playing here... {}", music_obj.count);
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
                                    playlist: "main".to_string(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::PlayPause,
                                }))
                                .await
                                .unwrap();
                        }
                        PungeCommand::SkipForwards => {
                            // so i guess the answer is doing .stop()? not .clear(). ig cause .stop() also clears the queue
                            let start = Instant::now();
                            music_obj.sink.stop();
                            println!("skip forards, top!!");
                            let old_id = music_obj.current_object.uniqueid.clone();
                            music_obj.count =
                                change_count(true, music_obj.count, music_obj.list.len());
                            music_obj.current_object =
                                music_obj.list[music_obj.count as usize].clone();

                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count as usize]
                                    .savelocationmp3
                                    .clone(),
                            ));
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.author.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: Some(old_id),
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: "main".to_string(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::SkippedForward,
                                }))
                                .await
                                .unwrap();
                            let end = Instant::now();
                            println!("time elapsed: {:?}", end.duration_since(start));
                        }
                        PungeCommand::SkipBackwards => {
                            music_obj.sink.stop();
                            music_obj.count =
                                change_count(false, music_obj.count, music_obj.list.len());
                            music_obj.current_object =
                                music_obj.list[music_obj.count as usize].clone();
                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count as usize]
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
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: None,
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: "main".to_string(),
                                    threshold: music_obj.current_object.threshold.clone(),
                                    context: Context::SkippedBackwards,
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
                            music_obj.count = index as isize;
                            music_obj.current_object =
                                music_obj.list[music_obj.count as usize].clone();
                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count as usize]
                                    .savelocationmp3
                                    .clone(),
                            ));
                            music_obj.to_play = true;
                            music_obj.sink.play();
                            sender
                                .send(ProgramCommands::NewData(MusicData {
                                    title: music_obj.current_object.title.clone(),
                                    author: music_obj.current_object.author.clone(),
                                    album: music_obj.current_object.author.clone(),
                                    song_id: music_obj.current_object.uniqueid.clone(),
                                    previous_id: None,
                                    volume: music_obj.sink.volume(),
                                    is_playing: true,
                                    shuffle: music_obj.shuffle,
                                    playlist: "main".to_string(),
                                    threshold: music_obj.current_object.threshold.clone(),
                                    context: Context::Seeked,
                                }))
                                .await
                                .unwrap();
                            // database_sender
                            //     .send(DatabaseMessages::Seeked(
                            //         music_obj.current_object.uniqueid.clone(),
                            //     ))
                            //     .unwrap();
                        }
                        PungeCommand::StaticVolumeUp => {
                            music_obj.sink.set_volume(music_obj.sink.volume() + 0.005);
                        }
                        PungeCommand::StaticVolumeDown => {
                            music_obj.sink.set_volume(music_obj.sink.volume() - 0.005);
                        }
                        PungeCommand::GoToAlbum => {
                            println!("going 2 album!")
                        }
                        PungeCommand::SkipToSeconds(val) => {
                            println!("skipping to seconds")
                        }
                        PungeCommand::ToggleShuffle => {
                            println!(
                                "imagine we are chaning shuffle status: {}",
                                &music_obj.current_object.title
                            );
                            if music_obj.shuffle {
                                music_obj.list = fetch::get_all_main().unwrap();
                                // it is shuffled, lets re-order
                                let index = music_obj // todo ok, need to put back in order
                                    .list
                                    .iter()
                                    .position(|r| {
                                        r.clone().uniqueid == music_obj.current_object.uniqueid
                                    })
                                    .unwrap();
                                println!("at inddex: {}", index);
                                music_obj.count = index as isize;
                                music_obj.shuffle = false;
                            } else {
                                let mut rng = rand::thread_rng();
                                music_obj.list.shuffle(&mut rng);
                                music_obj.shuffle = true;
                            }
                        }
                        PungeCommand::ChangePlaylist(name) => {
                            if name == "main".to_string() {
                                music_obj.list = fetch::get_all_main().unwrap();
                            } else {
                                let playlist_uuid = fetch::get_uuid_from_name(name);
                                music_obj.list =
                                    fetch::get_all_from_playlist(&playlist_uuid).unwrap();
                            }
                        }
                        PungeCommand::None => {
                            println!("is this even used?")
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
                        if music_obj.count < 0 {
                            music_obj.count =
                                (music_obj.list.len() as isize + music_obj.count) as isize;
                        }
                        if music_obj.count >= (music_obj.list.len() as isize) {
                            music_obj.count = 0;
                        }
                        if music_obj.sink.empty() {
                            println!("default appending!");
                            music_obj.sink.append(read_file_from_beginning(
                                music_obj.list[music_obj.count as usize]
                                    .savelocationmp3
                                    .clone(),
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
                                                    playlist: "main".to_string(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::SkippedForward,
                                                }))
                                                .await
                                                .unwrap();
                                            music_obj.sink.pause();
                                            music_obj.to_play = false
                                        }
                                        PungeCommand::SkipForwards => {
                                            let start = Instant::now();
                                            println!("skippin forrards");

                                            music_obj.sink.stop(); // why was this not here before and how did it even work !?
                                            let old_id = music_obj.current_object.uniqueid.clone();
                                            music_obj.count = change_count(
                                                true,
                                                music_obj.count,
                                                music_obj.list.len(),
                                            );
                                            music_obj.current_object =
                                                music_obj.list[music_obj.count as usize].clone();
                                            music_obj.sink.append(read_file_from_beginning(
                                                music_obj.list[music_obj.count as usize]
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
                                                    playlist: "main".to_string(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::SkippedForward,
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
                                                music_obj.count.clone(),
                                                music_obj.list.len(),
                                            );
                                            music_obj.current_object =
                                                music_obj.list[music_obj.count as usize].clone();
                                            if !music_obj.sink.is_paused() {
                                                // so this if stmt was on the upper match stmt, but kept causing problems with skipping and clearing the sink (even tho
                                                // the if occurs before the sink.append() ). so it only is down here, and seems to work just fine
                                                music_obj.sink.stop();
                                                music_obj.sink.clear()
                                            }
                                            music_obj.sink.append(read_file_from_beginning(
                                                music_obj.list[music_obj.count as usize]
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
                                                    playlist: "main".to_string(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::SkippedBackwards,
                                                }))
                                                .await
                                                .unwrap();
                                        }
                                        PungeCommand::NewVolume(val) => {
                                            music_obj.sink.set_volume((val as f32) / 80.0)
                                        }
                                        PungeCommand::ToggleShuffle => {
                                            if music_obj.shuffle {
                                                music_obj.list = fetch::get_all_main().unwrap();
                                                let index = music_obj
                                                    .list
                                                    .iter()
                                                    .position(|r| {
                                                        r.clone().uniqueid
                                                            == music_obj.current_object.uniqueid
                                                    })
                                                    .unwrap();
                                                println!("indexing: {}", index);
                                                music_obj.count = index as isize;
                                                music_obj.shuffle = false;
                                            } else {
                                                let mut rng = rand::thread_rng();
                                                music_obj.list.shuffle(&mut rng);
                                                music_obj.shuffle = true;
                                            }
                                        }
                                        PungeCommand::StaticVolumeUp => {
                                            music_obj
                                                .sink
                                                .set_volume(music_obj.sink.volume() + 0.005);
                                        }
                                        PungeCommand::StaticVolumeDown => {
                                            music_obj
                                                .sink
                                                .set_volume(music_obj.sink.volume() - 0.005);
                                        }
                                        PungeCommand::ChangeSong(uuid) => {
                                            let index = music_obj
                                                .list
                                                .iter()
                                                .position(|r| r.clone().uniqueid == uuid)
                                                .unwrap();

                                            music_obj.count = index as isize;
                                            music_obj.current_object =
                                                music_obj.list[music_obj.count as usize].clone();
                                            if !music_obj.sink.is_paused() {
                                                // so this if stmt was on the upper match stmt, but kept causing problems with skipping and clearing the sink (even tho
                                                // the if occurs before the sink.append() ). so it only is down here, and seems to work just fine
                                                music_obj.sink.stop();
                                                music_obj.sink.clear()
                                            }
                                            music_obj.sink.append(read_file_from_beginning(
                                                music_obj.list[music_obj.count as usize]
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
                                                    shuffle: music_obj.shuffle.clone(),
                                                    playlist: "main".to_string(),
                                                    threshold: music_obj.current_object.threshold,
                                                    context: Context::Seeked,
                                                }))
                                                .await
                                                .unwrap();
                                            // database_sender
                                            //     .send(DatabaseMessages::Seeked(
                                            //         music_obj.current_object.uniqueid.clone(),
                                            //     ))
                                            //     .unwrap();
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
                            music_obj.current_object =
                                music_obj.list[music_obj.count as usize].clone();
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
                                    playlist: "main".to_string(),
                                    threshold: music_obj.current_object.threshold,
                                    context: Context::AutoPlay,
                                }))
                                .await
                                .unwrap();
                            // database_sender
                            //     .send(DatabaseMessages::Played(
                            //         music_obj.current_object.uniqueid.clone(),
                            //     ))
                            //     .unwrap()
                        }
                    }
                }
                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
            }
        })
    }
}

// handles app events, used for listening for the window close event (for now)
fn handle_app_events(event: iced::Event, _status: iced::event::Status) -> Option<ProgramCommands> {
    match &event {
        iced::Event::Window(iced::window::Event::CloseRequested) => {
            Some(ProgramCommands::InAppEvent(AppEvent::CloseRequested))
        }
        _ => None,
    }
}

pub fn change_count(incrementing: bool, count: isize, vec_len: usize) -> isize {
    // change the count without worrying about index errors
    let new_count: isize = if count == 0 && !incrementing {
        // if removing and count =0 (would make it -1)
        // going below the limit
        (vec_len as isize) - 1
    } else if (count == (vec_len - 1) as isize) && incrementing {
        0 as isize // going above or equal the limit
    } else {
        if incrementing {
            // all other cases!
            count + 1
        } else {
            count - 1
        }
    };
    new_count
}
