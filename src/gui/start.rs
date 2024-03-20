// can we rename this to lib.rs at some point maybe??
use crate::db::fetch::{
    get_all_from_playlist, get_all_main, get_all_playlists, get_uuid_from_name,
};
use crate::db::insert::{add_to_playlist, create_playlist};
use crate::gui::messages::{AppEvent, Page, ProgramCommands, PungeCommand, TextType};
use crate::gui::persistent;
use crate::gui::table::{Column, ColumnKind, Row};
use crate::gui::{download_page, setting_page};
use crate::player::player_cache;
use crate::player::sort::get_values_from_db;
use crate::types::{Config, MusicData, UserPlaylist};
use crate::utils::backup::create_backup;
use crate::utils::cache;
use crate::utils::playlist::get_playlist;
use crate::utils::types;
use arc_swap::ArcSwap;
use std::sync::Arc;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

use iced::subscription::Subscription;
use iced::widget::{
    button, column, container, horizontal_space, pick_list, responsive, row, scrollable, slider,
    text, vertical_space,
};
use iced::Command;
use iced::{executor, Alignment, Application, Element, Length, Settings, Theme};
use tokio::sync::mpsc as async_sender; // does it need to be in scope?

pub fn begin() -> iced::Result {
    App::run(Settings {
        id: None,
        flags: (),
        window: iced::window::Settings {
            size: iced::Size {
                width: 1250.0,
                height: 700.0,
            },
            position: iced::window::Position::Default,
            min_size: Some(iced::Size {
                width: 1250.0,
                height: 700.0,
            }),
            max_size: Some(iced::Size {
                width: 2920.0,
                height: 2080.0,
            }),
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            level: iced::window::Level::Normal,
            icon: None, // will add soon i think
            platform_specific: iced::window::settings::PlatformSpecific {
                parent: None,
                drag_and_drop: false,
                skip_taskbar: false,
            },
            exit_on_close_request: false,
        },
        default_font: Default::default(),
        default_text_size: iced::Pixels { 0: 16.0 },
        antialiasing: false,
        fonts: Settings::<()>::default().fonts, // thanks source code?
    })
}
// pages for the gui

pub struct App {
    pub is_paused: bool,
    pub current_song: Arc<ArcSwap<MusicData>>, // represents title, auth, album, song_id, volume, shuffle, playlist
    sender: Option<async_sender::UnboundedSender<PungeCommand>>, // was not an option before !
    volume: u8,
    shuffle: bool,
    current_view: Page,
    download_page: crate::gui::download_page::DownloadPage,
    pub setting_page: setting_page::SettingPage, // pub so src\gui\subscrip can see the user choosen value increments
    media_page: crate::gui::media_page::MediaPage,
    playlist_page: crate::gui::new_playlist_page::PlaylistPage,
    download_list: Vec<types::Download>, // should also include the link somewhere to check for
    last_id: usize,
    manager: GlobalHotKeyManager, // TODO at some point: make interface for re-binding
    search: String,
    viewing_playlist: String, // could derive from cache soon... just the uniqueid rn
    side_menu_song_select: (String, String), // title, uniqueid. these two are for adding to playlists
    side_menu_playlist_select: (String, String), // title, uniqueid
    user_playlists: Vec<UserPlaylist>,
    // tarkah table stuff
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    columns: Vec<Column>,
    rows: Vec<Row>,
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
        let player_cache = player_cache::fetch_cache();
        (
            App {
                is_paused: true,
                current_song: Arc::new(ArcSwap::from_pointee(MusicData::default())),
                sender: None,
                volume: (player_cache.volume * 80.0) as u8, // 80 is out magic number from sink volume -> slider
                shuffle: player_cache.shuffle,
                current_view: Page::Main,
                download_page: download_page::DownloadPage::new(),
                setting_page: setting_page::SettingPage::new(),
                media_page: crate::gui::media_page::MediaPage::new(),
                playlist_page: crate::gui::new_playlist_page::PlaylistPage::new(None),
                download_list: vec![],
                last_id: 0,
                manager,
                search: "".to_string(),
                viewing_playlist: "main".to_string(),
                side_menu_song_select: ("".to_string(), "".to_string()),
                side_menu_playlist_select: ("".to_string(), "".to_string()),
                user_playlists: get_all_playlists().unwrap(), // im addicted to unwraping
                header: scrollable::Id::unique(),
                body: scrollable::Id::unique(),
                footer: scrollable::Id::unique(),
                columns: vec![
                    Column::new(ColumnKind::PlayButton),
                    Column::new(ColumnKind::Author),
                    Column::new(ColumnKind::Title),
                    Column::new(ColumnKind::Album),
                    Column::new(ColumnKind::Edit),
                ],
                rows: get_all_main()
                    .unwrap()
                    .into_iter()
                    .map(|item| Row {
                        title: item.title,
                        author: item.author,
                        album: item.album,
                        uniqueid: item.uniqueid,
                    })
                    .collect(), // get it from the other file lol
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
                self.current_song.store(Arc::new(data));
            }
            Self::Message::VolumeChange(val) => {
                self.volume = val;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::NewVolume(val))
                    .expect("failure sending msg");
            }
            Self::Message::ShuffleToggle => {
                self.shuffle = if self.shuffle { false } else { true };
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::ToggleShuffle)
                    .unwrap();
            }
            Self::Message::PlayToggle => {
                self.is_paused = if self.is_paused { false } else { true };
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::PlayOrPause)
                    .unwrap();
            }
            Self::Message::SkipForwards => {
                // if it is paused, and this is called, update the stop/play
                if self.is_paused {
                    self.is_paused = false;
                }
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::SkipForwards)
                    .unwrap();
            }
            Self::Message::SkipBackwards => {
                if self.is_paused {
                    self.is_paused = false;
                }
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::SkipBackwards)
                    .unwrap();
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
                    let playlist_title = if link.contains("list=") {
                        Some(get_playlist(link.as_str()).unwrap().title)
                    } else {
                        None
                    };
                    println!("pushing and downloading!");
                    self.download_list.push(types::Download {
                        id: self.last_id,
                        link: Some(link),
                        playlist_title,
                    });
                } else {
                    println!("not pushing !!! already downloading")
                }
            }
            Self::Message::AddToDownloadFeedback(feedback) => {
                // only is called from the subscription !!
                match feedback {
                    Some(item) => {
                        match item {
                            Ok(youtube_data) => {
                                println!(
                                    "{} made {:?}",
                                    &youtube_data.url,
                                    (&youtube_data.title, &youtube_data.author)
                                );
                                self.download_page.download_feedback.push(format!(
                                    "{} downloaded successfully!",
                                    format!("{} - {}", youtube_data.title, youtube_data.author)
                                ));
                                let mut ind = 0;
                                for (index, download) in self.download_list.iter().enumerate() {
                                    if download.link.as_ref().unwrap() == &youtube_data.url {
                                        println!("removed: {}", &youtube_data.url);
                                        ind = index;
                                    }
                                }
                                // not sure why this can be 0?
                                if ind != 0 {
                                    self.download_list.remove(ind);
                                }

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
                                if self.download_list.len() == 0 {
                                    self.download_page.download_feedback.push("Unexpected error (start.rs 271, download_list.len() == 0?)".to_string());
                                } else {
                                    self.download_page.text = String::from(""); // clear the textbox
                                    self.download_page.download_feedback.push(format!(
                                        "Error downloading {}: {:?}",
                                        self.download_list
                                            [self.download_list.len().saturating_sub(1)] // no underflow errors here buddy
                                        .link
                                        .clone()
                                        .unwrap(),
                                        error
                                    ));
                                    self.download_list
                                        .remove(self.download_list.len().saturating_sub(1));
                                }
                                // add to some list ? like failed downloads
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
                    let cache = player_cache::Cache {
                        song_id: lcl.song_id.clone(),
                        volume: lcl.volume,
                        shuffle: lcl.shuffle,
                        playlist: lcl.playlist.clone(),
                    };
                    player_cache::dump_cache(cache); // dumps user cache
                    println!("dumpepd cache!");

                    return iced::window::close::<Self::Message>(iced::window::Id::MAIN);
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
            Self::Message::PlaySong(song) => {
                // this is only used from the 'play' buttons on the songs
                if self.is_paused {
                    self.is_paused = false;
                }
                // if the viewing playlist is different than the most recent
                if self.current_song.load().playlist != self.viewing_playlist {
                    // change playlist, hopefully no data race occurs... if it does,
                    // we can change it to play the song, then change the playlist in the background..
                    self.sender
                        .as_ref()
                        .unwrap()
                        .send(PungeCommand::ChangePlaylist(self.viewing_playlist.clone()))
                        .unwrap();
                }
                // need to change active playlist, shuffle, and get index spot
                self.sender
                    .as_ref()
                    .unwrap()
                    .send(PungeCommand::ChangeSong(song))
                    .unwrap();
            }
            Self::Message::ChangeViewingPlaylist(playlist) => {
                // we will change the current view to the playlist view, and pass in the playlist to fill the content
                self.viewing_playlist = playlist.clone();

                // main should be treated just like a regular playlist !?
                if playlist == "main" {
                    let new = get_all_main().unwrap();
                    self.rows = new
                        .into_iter()
                        .map(|item| Row {
                            title: item.title,
                            author: item.author,
                            album: item.album,
                            uniqueid: item.uniqueid,
                        })
                        .collect();
                } else {
                    let new = get_all_from_playlist(playlist.as_str()).unwrap();
                    self.rows = new
                        .into_iter()
                        .map(|item| Row {
                            title: item.title,
                            author: item.author,
                            album: item.album,
                            uniqueid: item.uniqueid,
                        })
                        .collect();
                }
            }
            Self::Message::ChangeActivePlaylist(playlist) => {
                println!("changed active playlist! {}", &playlist.title);
                // so we will self.sender.send(PungeCommand::UpdateList()) or whatever. will try to search
            }
            Self::Message::SelectSong(uniqueid, song) => {
                // when the song is selected from the table, update the song in the top right
                self.side_menu_song_select = (song, uniqueid);
                // maybe buttons should bring title with it??? idk
            }
            Self::Message::PlaylistSelected(name) => {
                // set playlist uuid string to uniqueid and set side to the title of that uuid
                let uniqueid = get_uuid_from_name(name.clone());
                self.side_menu_playlist_select = (name, uniqueid);
            }
            Self::Message::AddToPlaylist(song_id, playlist_id) => {
                if song_id.is_none() | playlist_id.is_none() {
                    println!("fail!")
                } else {
                    add_to_playlist(playlist_id.unwrap(), song_id.unwrap()).unwrap(); // what abt duplicate addigs?
                                                                                      // clear the song once it was added :)
                    self.side_menu_song_select = ("".to_string(), "".to_string())
                }
            }
            Self::Message::ToggleList => {
                if self.rows.len() == 1 {
                    self.rows = get_all_main()
                        .unwrap()
                        .into_iter()
                        .map(|item| Row {
                            title: item.title,
                            author: item.author,
                            album: item.album,
                            uniqueid: item.uniqueid,
                        })
                        .collect();
                } else {
                    self.rows = vec![Row {
                        title: "This will".to_string(),
                        author: "Be fixed soon".to_string(),
                        album: "I promise".to_string(),
                        uniqueid: "".to_string(),
                    }]
                }
            }
            Self::Message::CreateBackup => {
                // get backup path from config and use it :)

                match create_backup(self.setting_page.backup_text.clone()) {
                    Ok(_) => {
                        println!("yippie!");
                    }
                    Err(e) => {
                        println!("whaa {:?}", e);
                    }
                };
            }
            Self::Message::UpdateWidgetText(text_type, txt) => match text_type {
                TextType::BackupText => {
                    self.setting_page.backup_text = txt;
                }
                TextType::Mp3Text => {
                    self.setting_page.mp3_path_text = txt;
                }
                TextType::JpgText => {
                    self.setting_page.jpg_path_text = txt;
                }
                TextType::StaticIncrement => {
                    self.setting_page.static_increment = txt;
                }
                TextType::StaticReduction => {
                    self.setting_page.static_reduction = txt;
                }
                TextType::UserTitle => {
                    self.playlist_page.user_title = txt;
                }
                TextType::UserDescription => {
                    self.playlist_page.user_description = txt;
                }
                TextType::UserThumbnail => {
                    self.playlist_page.user_thumbnail = txt;
                }
            },

            Self::Message::SaveConfig => {
                let static_increment = self
                    .setting_page
                    .static_increment
                    .clone()
                    .parse::<f32>()
                    .unwrap();
                let static_reduction = self
                    .setting_page
                    .static_reduction
                    .clone()
                    .parse::<f32>()
                    .unwrap();

                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::NewStatic(static_increment, static_reduction))
                    .unwrap();

                let obj = Config {
                    backup_path: self.setting_page.backup_text.clone(),
                    mp3_path: self.setting_page.mp3_path_text.clone(),
                    jpg_path: self.setting_page.jpg_path_text.clone(),
                    static_increment,
                    static_reduction,
                };
                match cache::write_to_cache(obj) {
                    Ok(t) => {
                        println!("config written successfully")
                    }
                    Err(e) => {
                        println!("Config failed! {:?}", e)
                    }
                }
            }
            Self::Message::NewPlaylist => {
                // TODO we should be doing a check for updating an existing playlist vs making a new one
                let playlist = UserPlaylist::new(
                    self.playlist_page.user_title.clone(),
                    self.playlist_page.user_description.clone(),
                    self.playlist_page.user_thumbnail.clone(),
                    false,
                );
                create_playlist(playlist).unwrap();
                self.user_playlists = get_all_playlists().unwrap();
                // also refresh the buttons!
            }

            _ => println!("inumplmented"),
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let search_container = container(row![
            iced::widget::text_input("GoTo closest match", self.search.as_str())
                .on_input(ProgramCommands::UpdateSearch)
                .width(Length::Fixed(250.0)),
            button(text("Confirm")).on_press(ProgramCommands::GoToSong)
        ]);
        let table = responsive(|size| {
            let table = iced_table::table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                ProgramCommands::SyncHeader,
            );
            table.into()
        });

        let mut all_playlists_but_main = self.user_playlists.clone();
        // user should always have the 'main' playlist

        all_playlists_but_main.remove(0);
        let actions_cont = container(column![
            text(self.side_menu_song_select.0.clone()),
            button(text("Add to:")).on_press(ProgramCommands::AddToPlaylist(
                Some(self.side_menu_song_select.1.clone()),
                Some(self.side_menu_playlist_select.1.clone())
            )),
            // need to have main not show up here that makes no sense..
            pick_list(
                all_playlists_but_main
                    .iter()
                    .map(|pl| pl.title.clone())
                    .collect::<Vec<String>>(),
                Some(self.side_menu_playlist_select.0.clone()),
                ProgramCommands::PlaylistSelected
            ),
        ])
        .padding(15);
        let buttons: Vec<Element<ProgramCommands>> = self
            .user_playlists
            .iter()
            .map(|playlist| {
                button(text(playlist.title.clone()))
                    .on_press(ProgramCommands::ChangeViewingPlaylist(
                        playlist.uniqueid.clone(),
                    ))
                    .height(Length::Fixed(32.5)) // playlist button height :)
                    .into()
            })
            .collect();
        let table_cont = container(table).height(Length::Fixed(540.0)).padding(20);

        let curr_song = self.current_song.load();
        let main_page_2 = container(row![column![
            row![
                row![
                    persistent::render_top_buttons(Page::Main),
                    button(text("Toggle table")).on_press(ProgramCommands::ToggleList)
                ]
                .spacing(10),
                horizontal_space(),
                // self.render_sidebar()
            ],
            row![
                table_cont,
                column![
                    actions_cont,
                    iced::widget::Column::with_children(buttons).spacing(10.0)
                ]
            ],
            // vertical_space(), // puts space between the main content (inc. sidebar) and the bottom controls
            row![
                column![
                    text(curr_song.title.clone()),
                    text(curr_song.author.clone()),
                    text(curr_song.album.clone())
                ]
                .padding(2.5)
                .width(225.0),
                button(text("Go back")).on_press(ProgramCommands::SkipBackwards),
                button(text(if self.is_paused { "Play!" } else { "Stop!" }))
                    .on_press(ProgramCommands::PlayToggle),
                button(text("Go forwards")).on_press(ProgramCommands::SkipForwards),
                button(text(format!(
                    "Shuffle ({})",
                    if self.shuffle { "On" } else { "Off" }
                )))
                .on_press(ProgramCommands::ShuffleToggle),
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
            vertical_space()
        ],]);
        match self.current_view {
            // which page to display
            // Page::Main => row![main_page, self.render_sidebar()].into(), // this format makes it a bit easier to deal with all contents
            Page::Main => main_page_2.into(),
            Page::Download => self.download_page.view(),
            Page::Settings => self.setting_page.view(),
            Page::Media => self.media_page.view(),
            Page::Playlist => self.playlist_page.view(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::subscription::Subscription::batch(vec![
            self.music_loop(),
            self.hotkey_loop(),
            Subscription::batch(self.download_list.iter().map(types::Download::subscription)),
            self.database_subscription(self.current_song.clone()),
            self.close_app_sub(),
            // self.database_sub(database_receiver),
        ]) // is two batches required?? prolly not
    }
}
