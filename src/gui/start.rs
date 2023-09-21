// can we rename this to lib.rs at some point maybe??
use crate::gui::messages::{Page, ProgramCommands, PungeCommand};
use crate::gui::subscription as sub;
use crate::player::interface;
use crate::{gui, playliststructs};
use iced::widget::{button, column, container, row, slider, text};
use iced::Command;
use iced::{
    executor, Alignment, Application, Color, Element, Error, Length, Sandbox, Settings, Theme,
};
use std::thread;

use crate::db::fetch;
use crate::player::interface::{read_file_from_beginning, MusicPlayer};
use crate::playliststructs::PungeMusicObject;
use crate::utils::{types, youtube_interface};
use iced::futures::channel::mpsc::Sender;
use iced::futures::sink::SinkExt;
use iced::subscription::{self, Subscription};
use rand::seq::SliceRandom;
use std::sync::mpsc;
use tokio::sync::mpsc as async_sender; // does it need to be in scope?
use tokio::sync::mpsc::UnboundedReceiver; // allow the playlist to be shuffled

pub fn begin() -> iced::Result {
    App::run(Settings::default())
}
// pages for the gui
use crate::gui::{download_page, setting_page};
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};

pub struct App {
    theme: Theme,
    is_paused: bool,
    current_song: (String, String, String),
    sender: Option<async_sender::UnboundedSender<PungeCommand>>, // was not an option before !
    volume: u8,
    current_view: Page,
    download_page: crate::gui::download_page::DownloadPage,
    setting_page: setting_page::SettingPage,
    download_list: Vec<types::Download>, // should also include the link somewhere to check for
    last_id: usize,
    manager: GlobalHotKeyManager,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = ProgramCommands;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (App, iced::Command<Self::Message>) {
        // hotkey management and this is where new keybinds are to be added
        let manager = GlobalHotKeyManager::new().unwrap();
        let hotkey_1 = HotKey::new(Some(Modifiers::CONTROL), Code::ArrowRight);
        let hotkey_2 = HotKey::new(Some(Modifiers::CONTROL), Code::ArrowLeft);
        let hotkey_3 = HotKey::new(Some(Modifiers::CONTROL), Code::End);
        let hotkey_4 = HotKey::new(Some(Modifiers::CONTROL), Code::PageDown);
        let hotkey_5 = HotKey::new(Some(Modifiers::CONTROL), Code::ArrowUp);
        let hotkey_6 = HotKey::new(Some(Modifiers::CONTROL), Code::ArrowDown);
        manager.register(hotkey_1).unwrap();
        manager.register(hotkey_2).unwrap();
        manager.register(hotkey_3).unwrap();
        manager.register(hotkey_4).unwrap();
        manager.register(hotkey_5).unwrap();
        manager.register(hotkey_6).unwrap();
        (
            App {
                theme: Default::default(),
                is_paused: true,
                current_song: ("".to_string(), "".to_string(), "".to_string()),
                sender: None,
                volume: 25,
                current_view: Page::Main,
                download_page: download_page::DownloadPage::new(),
                setting_page: setting_page::SettingPage::new(),
                download_list: vec![],
                last_id: 0,
                manager,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Punge!!")
    }

    fn update(&mut self, msg: Self::Message) -> iced::Command<ProgramCommands> {
        match msg {
            Self::Message::Test => {
                println!("doing play, here?");
                //self.sender.as_mut().unwrap().send(PungeCommand::Play);  // does it work?
                // self.sender.send(Command::Play).unwrap();  // i dont think this unwrap() can fail ..
            }
            Self::Message::Send(cmd) => {
                println!("sending punge cmd: {:?}", &cmd);
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(cmd)
                    .expect("failure sending msg");
            }
            Self::Message::UpdateSender(sender) => {
                println!("updated sender!");
                self.sender = sender;
            }
            Self::Message::NewData(art, title, alb) => {
                println!("The new information given to update: {art} {title} {alb}");
                self.current_song = (art, title, alb)
            }
            Self::Message::VolumeChange(val) => {
                self.volume = val;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::NewVolume(val))
                    .expect("failure sending msg");
            }
            Self::Message::DownloadLink(link) => {
                println!("imagine we download {} here", &link);
                // from here, we will match and add the result into a 'feedback box'
            }
            Self::Message::ChangePage(page) => self.current_view = page,
            Self::Message::UpdateDownloadEntry(string) => {
                self.download_page.text = string;
            }
            Self::Message::Download(link) => {
                // should be depreciated?
                let mut eval: bool = false;
                for download in self.download_list.iter() {
                    if &download.link.clone().unwrap() == &link {
                        self.download_page
                            .download_feedback
                            .push(format!("Download already started on {}", link.clone()));
                        eval = true;
                    }
                }
                if eval == false {
                    println!("pushing and downloading!");
                    self.download_list.push(types::Download {
                        id: self.last_id,
                        link: Some(link),
                    });
                } else {
                    println!("not pushing !!! already downloading")
                }
            }
            Self::Message::AddToDownloadFeedback(feedback) => {
                // only is called from the subscription !!
                match feedback {
                    Some(t) => {
                        for item in t {
                            match item {
                                Ok((link, auth_and_title)) => {
                                    println!("{} made {:?}", &link, &auth_and_title);
                                    self.download_page.download_feedback.push(format!(
                                        "{} downloaded successfully!",
                                        auth_and_title
                                    ));
                                    let mut ind = 0;
                                    for (index, download) in self.download_list.iter().enumerate() {
                                        if download.link.as_ref().unwrap() == &link {
                                            println!("removed: {}", &link);
                                            ind = index;
                                        }
                                    }
                                    self.download_list.remove(ind);
                                }
                                Err(error) => {
                                    self.download_page.download_feedback.push(format!(
                                        "Error downloading {}: {:?}",
                                        self.download_list[self.download_list.len() - 1]
                                            .link
                                            .clone()
                                            .unwrap(),
                                        error
                                    ))
                                    // add to some list ? like failed downloads
                                }
                            }
                        }
                    }
                    None => {
                        println!("start.rs: none after downloadfeedback?? 128")
                    }
                }
            }
            Self::Message::Debug => {
                println!("Da list: {:?}", self.download_list)
            }

            _ => println!("inumplmented"),
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let page_buttons = row![
            button(text("Settings")).on_press(ProgramCommands::ChangePage(Page::Settings)),
            button(text("Download!")).on_press(ProgramCommands::ChangePage(Page::Download)),
        ]
        .spacing(50);
        let main_page = container(column![
            page_buttons,
            row![
                column![
                    text(self.current_song.0.clone()),
                    text(self.current_song.1.clone()),
                    text(self.current_song.2.clone())
                ],
                button(text("Go back"))
                    .on_press(ProgramCommands::Send(PungeCommand::SkipBackwards)),
                button(text("Play / Pause"))
                    .on_press(ProgramCommands::Send(PungeCommand::PlayOrPause)),
                button(text("Go forwards"))
                    .on_press(ProgramCommands::Send(PungeCommand::SkipForwards)),
                button(text("Shuffle"))
                    .on_press(ProgramCommands::Send(PungeCommand::ToggleShuffle)),
                slider(0..=100, self.volume, Self::Message::VolumeChange).width(150)
            ]
            .spacing(50)
            .padding(iced::Padding::new(10 as f32))
        ]);
        match self.current_view {
            // which page to display
            Page::Main => main_page.into(),
            Page::Download => self.download_page.view(),
            Page::Settings => self.setting_page.view(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let hotkey_loop = iced::subscription::channel(5, 32, |mut sender| async move {
            use iced::futures::sink::SinkExt;
            loop {
                match GlobalHotKeyEvent::receiver().try_recv() {
                    Ok(hotkey) => {
                        // handle global keybinds
                        println!("new keybind incming: {:?}", hotkey);
                        match hotkey {
                            GlobalHotKeyEvent { id: 4121890298 } => {
                                // right arrow
                                sender
                                    .send(Self::Message::Send(PungeCommand::SkipForwards))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 2037224482 } => {
                                // up arrow
                                sender
                                    .send(Self::Message::Send(PungeCommand::StaticVolumeUp))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 1912779161 } => {
                                // left arrow??
                                sender
                                    .send(Self::Message::Send(PungeCommand::SkipBackwards))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 4174001518 } => {
                                // down arrow!
                                sender
                                    .send(Self::Message::Send(PungeCommand::StaticVolumeDown))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 3520754938 } => {
                                // page down (shuffle)
                                sender
                                    .send(Self::Message::Send(PungeCommand::ToggleShuffle))
                                    .await
                                    .unwrap();
                            }
                            GlobalHotKeyEvent { id: 3009842507 } => {
                                // end (pause)
                                sender
                                    .send(Self::Message::Send(PungeCommand::PlayOrPause))
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
        });
        use iced::futures::SinkExt;
        let music_loop = iced::subscription::channel(0, 32, |mut sender| async move {
            let (gui_send, mut gui_rec) = tokio::sync::mpsc::unbounded_channel();
            sender
                .send(Self::Message::UpdateSender(Some(gui_send)))
                .await
                .unwrap(); // send the sender to the gui !!
            let items: Vec<PungeMusicObject> = fetch::get_all_main().unwrap();
            // maybe here  we need to get index of last song that was on?
            // send the data to the program
            let mut music_obj = interface::MusicPlayer::new(items);
            sender
                .send(Self::Message::NewData(
                    music_obj.current_object.title.clone(),
                    music_obj.current_object.author.clone(),
                    music_obj.current_object.album.clone(),
                ))
                .await
                .unwrap();

            // main music loop!
            println!("starting main loop");
            loop {
                match gui_rec.try_recv() {
                    Ok(cmd) => match cmd {
                        PungeCommand::PlayOrPause => {
                            if music_obj.sink.is_paused() || music_obj.sink.empty() {
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
                            } else {
                                println!("stooping here! (top)");
                                music_obj.sink.pause();
                                music_obj.to_play = false
                            }
                        }
                        PungeCommand::SkipForwards => {
                            println!("skip forards, top!!");
                            music_obj.sink.stop();
                            music_obj.count =
                                change_count(true, music_obj.count.clone(), music_obj.list.len());
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
                                .send(ProgramCommands::NewData(
                                    music_obj.list[music_obj.count as usize].title.clone(),
                                    music_obj.list[music_obj.count as usize].author.clone(),
                                    music_obj.list[music_obj.count as usize].album.clone(),
                                ))
                                .await
                                .unwrap();
                        }
                        PungeCommand::SkipBackwards => {
                            music_obj.sink.stop();
                            music_obj.count =
                                change_count(false, music_obj.count.clone(), music_obj.list.len());
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
                                .send(ProgramCommands::NewData(
                                    music_obj.list[music_obj.count as usize].title.clone(),
                                    music_obj.list[music_obj.count as usize].author.clone(),
                                    music_obj.list[music_obj.count as usize].album.clone(),
                                ))
                                .await
                                .unwrap();
                        }
                        PungeCommand::NewVolume(val) => {
                            music_obj.sink.set_volume((val as f32) / 80.0)
                        }
                        PungeCommand::ChangeSong(index) => {
                            println!("index for song: {}", index);
                            // also change music_obj.current_object
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
                            println!("changing name!!");
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
                                            if music_obj.sink.is_paused() || music_obj.sink.empty()
                                            {
                                                // we are going to play
                                                if !music_obj.sink.is_paused()
                                                    && music_obj.sink.empty()
                                                {
                                                    let song = interface::read_file_from_beginning(
                                                        music_obj.list[music_obj.count as usize]
                                                            .savelocationmp3
                                                            .clone(),
                                                    );
                                                    sender
                                                        .send(ProgramCommands::NewData(
                                                            music_obj.list
                                                                [music_obj.count as usize]
                                                                .title
                                                                .clone(),
                                                            music_obj.list
                                                                [music_obj.count as usize]
                                                                .author
                                                                .clone(),
                                                            music_obj.list
                                                                [music_obj.count as usize]
                                                                .album
                                                                .clone(),
                                                        ))
                                                        .await
                                                        .unwrap();
                                                    music_obj.sink.append(song);
                                                }

                                                music_obj.to_play = true;
                                                music_obj.sink.play();
                                                sender
                                                    .send(ProgramCommands::NewData(
                                                        music_obj.list[music_obj.count as usize]
                                                            .title
                                                            .clone(),
                                                        music_obj.list[music_obj.count as usize]
                                                            .author
                                                            .clone(),
                                                        music_obj.list[music_obj.count as usize]
                                                            .album
                                                            .clone(),
                                                    ))
                                                    .await
                                                    .unwrap();
                                            } else {
                                                println!("stooping here! (bottom)");
                                                music_obj.sink.pause();
                                                music_obj.to_play = false
                                            }
                                        }
                                        PungeCommand::SkipForwards => {
                                            println!("skippin forrards");
                                            music_obj.count = change_count(
                                                true,
                                                music_obj.count.clone(),
                                                music_obj.list.len(),
                                            );
                                            music_obj.current_object =
                                                music_obj.list[music_obj.count as usize].clone();
                                            if !music_obj.sink.is_paused() {
                                                // wait is this even required cause this can only be hit in the 'active palying' loop?
                                                music_obj.sink.stop(); // stop
                                                music_obj.sink.clear() // clear the sink of current song
                                            }
                                            music_obj.sink.append(read_file_from_beginning(
                                                music_obj.list[music_obj.count as usize]
                                                    .savelocationmp3
                                                    .clone(),
                                            ));
                                            music_obj.to_play = true;
                                            music_obj.sink.play();
                                            sender
                                                .send(ProgramCommands::NewData(
                                                    music_obj.list[music_obj.count as usize]
                                                        .title
                                                        .clone(),
                                                    music_obj.list[music_obj.count as usize]
                                                        .author
                                                        .clone(),
                                                    music_obj.list[music_obj.count as usize]
                                                        .album
                                                        .clone(),
                                                ))
                                                .await
                                                .unwrap();
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
                                                .send(ProgramCommands::NewData(
                                                    music_obj.list[music_obj.count as usize]
                                                        .title
                                                        .clone(),
                                                    music_obj.list[music_obj.count as usize]
                                                        .author
                                                        .clone(),
                                                    music_obj.list[music_obj.count as usize]
                                                        .album
                                                        .clone(),
                                                ))
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
                                change_count(true, music_obj.count.clone(), music_obj.list.len());
                            music_obj.current_object =
                                music_obj.list[music_obj.count as usize].clone();
                            // new info :P
                            sender
                                .send(ProgramCommands::NewData(
                                    music_obj.list[music_obj.count as usize].title.clone(),
                                    music_obj.list[music_obj.count as usize].author.clone(),
                                    music_obj.list[music_obj.count as usize].album.clone(),
                                ))
                                .await
                                .unwrap();
                        }
                    }
                }
                async_std::task::sleep(std::time::Duration::from_millis(50)).await;
            }
        });
        // so this could eventually mimic the iced-rs/iced/blob/0.10/examples/download_progress, but i dont want to impl that, it would take so long
        // and im pretty sure that rustube has async callback for download progress, so it should be possible.

        // will also need to implement keybinds here. will do at another time though

        iced::subscription::Subscription::batch(vec![
            music_loop,
            hotkey_loop,
            Subscription::batch(self.download_list.iter().map(types::Download::subscription)),
        ]) // is two batches required?? prolly not
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
