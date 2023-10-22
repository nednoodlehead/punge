// can we rename this to lib.rs at some point maybe??
use crate::gui::messages::AppEvent;
use crate::gui::messages::{DatabaseMessages, Page, ProgramCommands, PungeCommand};
use crate::gui::{download_page, setting_page};
use crate::player::cache;
use crate::player::interface;
use crate::player::interface::{read_file_from_beginning, MusicPlayer};
use crate::player::sort::get_values_from_db;
use crate::playliststructs::MusicData;
use crate::playliststructs::PungeMusicObject;
use crate::utils::{types, youtube_interface};
use crate::{gui, playliststructs};
use arc_swap::{ArcSwap, ArcSwapAny};
use async_std::task;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use iced::futures::channel::mpsc::{Sender, UnboundedReceiver, UnboundedSender};
use iced::futures::sink::SinkExt;
use iced::subscription::{self, Subscription};
use iced::widget::{
    button, column, container, horizontal_space, row, slider, text, vertical_space,
};
use iced::Command;
use iced::{
    executor, Alignment, Application, Color, Element, Error, Event, Length, Settings, Theme,
};
use rand::seq::SliceRandom;
use std::sync::Arc;
use tokio::sync::mpsc as async_sender; // does it need to be in scope?

pub fn begin() -> iced::Result {
    App::run(Settings {
        id: None,
        flags: (),
        window: iced::window::Settings {
            size: (1150, 700),
            position: iced::window::Position::Default,
            min_size: Some((1150, 700)),
            max_size: Some((2920, 2080)),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: iced::window::Level::Normal,
            icon: None, // will add soon i think
            platform_specific: iced::window::PlatformSpecific {
                parent: None,
                drag_and_drop: false,
            },
        },
        default_font: Default::default(),
        default_text_size: 16.0,
        antialiasing: false,
        exit_on_close_request: false, // so we can save the data before exiting
    })
}
// pages for the gui

pub struct App {
    theme: Theme,
    pub is_paused: bool,
    pub current_song: Arc<ArcSwap<Arc<MusicData>>>, // represents title, auth, album, song_id, volume, shuffle, playlist
    sender: Option<async_sender::UnboundedSender<PungeCommand>>, // was not an option before !
    volume: u8,
    current_view: Page,
    download_page: crate::gui::download_page::DownloadPage,
    setting_page: setting_page::SettingPage,
    media_page: crate::gui::media_page::MediaPage,
    download_list: Vec<types::Download>, // should also include the link somewhere to check for
    last_id: usize,
    manager: GlobalHotKeyManager,
    search: String,
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
                current_song: Arc::new(ArcSwap::new(Arc::new(Arc::new(MusicData::default())))),
                sender: None,
                volume: 25,
                current_view: Page::Main,
                download_page: download_page::DownloadPage::new(),
                setting_page: setting_page::SettingPage::new(),
                media_page: crate::gui::media_page::MediaPage::new(),
                download_list: vec![],
                last_id: 0,
                manager,
                search: "".to_string(),
            },
            Command::none(),
        )
    }
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Punge!!")
    }

    fn update(&mut self, msg: Self::Message) -> iced::Command<ProgramCommands> {
        println!("MATCHING MSG: {:?}", &msg);
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
            Self::Message::NewData(data) => {
                println!(
                    "The new information given to update: {} {} {}",
                    data.author, data.title, data.album
                );
                self.current_song.store(Arc::new(Arc::new(data)));
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
                // remove at some point? post done optimziing i guess
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
                                    if self.current_song.load().playlist == "main".to_string() {
                                        println!("sender status?: {:?}", self.sender);
                                        // if main is the current playlist, refresh it so the new song shows up
                                        self.sender
                                            .as_mut()
                                            .unwrap()
                                            .send(PungeCommand::ChangePlaylist("main".to_string()))
                                            .unwrap();
                                    }
                                }
                                Err(error) => {
                                    self.download_page.download_feedback.push(format!(
                                        "Error downloading {}: {:?}",
                                        self.download_list
                                            [self.download_list.len().saturating_sub(1)]
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
            Self::Message::InAppEvent(t) => match t {
                AppEvent::CloseRequested => {
                    let lcl = self.current_song.load();
                    let cache = cache::Cache {
                        song_id: lcl.song_id.clone(),
                        volume: lcl.volume,
                        shuffle: lcl.shuffle,
                        playlist: lcl.playlist.clone(),
                    };
                    cache::dump_cache(cache); // dumps user cache
                    println!("dumpepd cache!");

                    return iced::window::close::<Self::Message>();
                }
            },
            Self::Message::UpdateSearch(input) => {
                self.search = input;
            }

            Self::Message::GoToSong => {
                let val = get_values_from_db(
                    self.current_song.load().playlist.clone(),
                    self.search.clone(),
                );
                println!("GoToSong: {:?}", val);
                if val.is_empty() {
                    // if the user's search gives no results, tell them in the search box
                    self.search = format!("{} returned no results", self.search);
                } else {
                    self.sender
                        .as_ref()
                        .unwrap()
                        .send(PungeCommand::ChangeSong(
                            val[val.len() - 1].clone().1.uniqueid,
                        ))
                        .unwrap();
                    self.search = "".to_string()
                }
            }
            Self::Message::ChangeViewingPlaylist(playlist) => {
                // we will change the current view to the playlist view, and pass in the playlist to fill the content
                println!("received msg for playlist: {}", &playlist.title);
            }
            Self::Message::ChangeActivePlaylist(playlist) => {
                println!("changed active playlist! {}", &playlist.title);
                // so we will self.sender.send(PungeCommand::UpdateList()) or whatever. will try to search
            }

            _ => println!("inumplmented"),
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let page_buttons: iced::widget::Row<'_, ProgramCommands> = row![
            button(text("Settings")).on_press(ProgramCommands::ChangePage(Page::Settings)),
            button(text("Download!")).on_press(ProgramCommands::ChangePage(Page::Download)),
        ]
        .spacing(50);
        let search_container = container(row![
            iced::widget::text_input("GoTo closest match", self.search.as_str())
                .on_input(ProgramCommands::UpdateSearch)
                .width(Length::Fixed(250.0)),
            button(text("Confirm")).on_press(ProgramCommands::GoToSong)
        ]);
        let curr_song = self.current_song.load();
        let main_page_2 = container(row![column![
            row![
                row![text("main area content?"), page_buttons],
                horizontal_space(Length::Fill),
                self.render_sidebar()
            ],
            vertical_space(Length::Fill), // puts space between the main content (inc. sidebar) and the bottom controls
            row![
                column![
                    text(curr_song.title.clone()),
                    text(curr_song.author.clone()),
                    text(curr_song.album.clone())
                ]
                .padding(2.5)
                .width(225.0),
                button(text("Go back"))
                    .on_press(ProgramCommands::Send(PungeCommand::SkipBackwards)),
                button(text("Play / Pause"))
                    .on_press(ProgramCommands::Send(PungeCommand::PlayOrPause)),
                button(text("Go forwards"))
                    .on_press(ProgramCommands::Send(PungeCommand::SkipForwards)),
                button(text("Shuffle"))
                    .on_press(ProgramCommands::Send(PungeCommand::ToggleShuffle)),
                column![
                    slider(0..=30, self.volume, Self::Message::VolumeChange).width(150),
                    search_container
                ]
                .align_items(Alignment::Center)
                .spacing(5.0)
            ]
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(50.0),
            vertical_space(Length::Fixed(30.0))
        ],]);
        match self.current_view {
            // which page to display
            // Page::Main => row![main_page, self.render_sidebar()].into(), // this format makes it a bit easier to deal with all contents
            Page::Main => main_page_2.into(),
            Page::Download => self.download_page.view(),
            Page::Settings => self.setting_page.view(),
            Page::Media => self.media_page.view(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // let (database_sender, database_receiver): (
        //     async_sender::UnboundedSender<ProgramCommands>,
        //     async_sender::UnboundedReceiver<ProgramCommands>,
        // ) = tokio::sync::mpsc::unbounded_channel();

        iced::subscription::Subscription::batch(vec![
            self.music_loop(),
            self.hotkey_loop(),
            Subscription::batch(self.download_list.iter().map(types::Download::subscription)),
            self.close_app_sub(),
            // self.database_sub(database_receiver),
        ]) // is two batches required?? prolly not
    }
}
