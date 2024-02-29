// can we rename this to lib.rs at some point maybe??
use crate::db::fetch::{get_all_main, get_all_playlists, get_name_from_uuid, get_uuid_from_name};
use crate::db::insert::add_to_playlist;
use crate::gui::messages::AppEvent;
use crate::gui::messages::{Page, ProgramCommands, PungeCommand};
use crate::gui::table::{Column, ColumnKind, Row};
use crate::gui::{download_page, setting_page};
use crate::player::cache;
use crate::player::sort::get_values_from_db;
use crate::playliststructs::{MusicData, UserPlaylist};
use iced_table::table;

use crate::utils::types;

use arc_swap::ArcSwap;

use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

use iced::futures::sink::SinkExt;
use iced::subscription::Subscription;
use iced::widget::{
    button, column, container, horizontal_space, pick_list, responsive, row, scrollable, slider,
    text, vertical_space,
};
use iced::Command;
use iced::{executor, Alignment, Application, Element, Length, Settings, Theme};
use std::sync::Arc;
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
    theme: Theme, // for changing the theme at a later time
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
    manager: GlobalHotKeyManager, // TODO at some point: make interface for re-binding
    search: String,
    selected_uuid: Option<String>,  // ok ngl there gotta be a better way to handle this..  TODO
    playlist_uuid: Option<String>,  // like a vec that holds all song id's, that is added by checkboxes
    side_menu_playlist_select: String,  // will refactor soon !
    selected_song_name: String,
    user_playlists: Vec<UserPlaylist>,
    add_to_playlist_feedback: String,
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
                selected_uuid: None,
                playlist_uuid: None,
                side_menu_playlist_select: String::from(""),
                selected_song_name: String::from(""),
                add_to_playlist_feedback: String::from(""),
                user_playlists: get_all_playlists().unwrap(),  // im addicted to unwraping
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
                                        self.download_page.download_feedback.push("Unexpected error (start.rs 222, download_list.len() == 0?)".to_string());
                                    } else {
                                        self.download_page.download_feedback.push(format!(
                                            "Error downloading {}: {:?}",
                                            self.download_list
                                                [self.download_list.len().saturating_sub(1)] // no underflow errors here buddy
                                            .link
                                            .clone()
                                            .unwrap(),
                                            error
                                        ))
                                    }
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
            Self::Message::ChangeViewingPlaylist(playlist) => {
                // we will change the current view to the playlist view, and pass in the playlist to fill the content
                println!("received msg for playlist: {}", &playlist.title);
            }
            Self::Message::ChangeActivePlaylist(playlist) => {
                println!("changed active playlist! {}", &playlist.title);
                // so we will self.sender.send(PungeCommand::UpdateList()) or whatever. will try to search
            }
            Self::Message::SelectSong(uniqueid, song) => {
                // when the song is selected from the table, update the song in the top right
                self.selected_uuid = Some(uniqueid);
                self.selected_song_name = song;
                // maybe buttons should bring title with it??? idk
            }
            Self::Message::PlaylistSelected(name) => {
                // set playlist uuid string to uniqueid and set side to the title of that uuid
                let uniqueid = get_uuid_from_name(name.clone());
                self.playlist_uuid = Some(uniqueid);
                self.side_menu_playlist_select = name;
                
            }
            Self::Message::AddToPlaylist(song_id, playlist_id) => {
                if song_id.is_none() | playlist_id.is_none() {
                    println!("fail!")
                }
                else {
                add_to_playlist(playlist_id.unwrap(), song_id.unwrap()); // what abt duplicate addigs?
                }
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

        let actions_cont = container(column![text(self.selected_song_name.clone()),
             pick_list(self.user_playlists.iter().map(|pl| pl.title.clone()).collect::<Vec<String>>(), Some(self.side_menu_playlist_select.clone()),
             ProgramCommands::PlaylistSelected), button(text("Add to:")).on_press(ProgramCommands::AddToPlaylist(self.selected_uuid.clone(), self.playlist_uuid.clone()))]).padding(15);

        let table_cont = container(table).height(Length::Fixed(540.0)).padding(20);

        let curr_song = self.current_song.load();
        let main_page_2 = container(row![column![
            row![
                row![page_buttons],
                horizontal_space(),
                // self.render_sidebar()
            ],
            row![table_cont, actions_cont],
            // vertical_space(), // puts space between the main content (inc. sidebar) and the bottom controls
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
            vertical_space()
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
