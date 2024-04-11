// can we rename this to lib.rs at some point maybe??
use crate::db::fetch::{
    get_all_from_playlist, get_all_main, get_all_playlists, get_uuid_from_name, song_from_uuid,
};
use crate::db::insert::{add_empty_entries, add_to_playlist, create_playlist};
use crate::db::update::{delete_from_playlist, update_song};
use crate::gui::messages::{AppEvent, Page, ProgramCommands, PungeCommand, TextType};
use crate::gui::persistent;
use crate::gui::table::{Column, ColumnKind, Row};
use crate::gui::{download_page, setting_page};
use crate::player::player_cache;
use crate::player::sort::get_values_from_db;
use crate::types::{Config, MusicData, UserPlaylist};
use crate::utils::backup::create_backup;
use crate::utils::cache;
use crate::utils::delete::delete_record_and_file;
use crate::utils::playlist::get_playlist;
use crate::yt::interface::download_interface;
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
        default_text_size: iced::Pixels(16.0),
        antialiasing: false,
        fonts: Settings::<()>::default().fonts, // thanks source code?
    })
}
// pages for the gui

pub struct App {
    pub is_paused: bool,
    pub current_song: Arc<ArcSwap<MusicData>>, // represents title, auth, album, song_id, volume, shuffle, playlist
    sender: Option<async_sender::UnboundedSender<PungeCommand>>, // was not an option before !
    pub volume: u8,
    pub shuffle: bool,
    pub scrubber: u32,
    pub time_elapsed: u32,
    pub total_time: u32,
    current_view: Page,
    download_page: crate::gui::download_page::DownloadPage,
    pub setting_page: setting_page::SettingPage, // pub so src\gui\subscrip can see the user choosen value increments
    media_page: crate::gui::media_page::MediaPage,
    playlist_page: crate::gui::new_playlist_page::PlaylistPage,
    song_edit_page: crate::gui::song_edit_page::SongEditPage,
    download_list: Vec<String>, // full link, songs are removed when finished / errored. Used so multiple downloads are not started
    last_id: usize,
    manager: GlobalHotKeyManager, // TODO at some point: make interface for re-binding
    pub search: String,
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
        let config_cache = match cache::read_from_cache() {
            Ok(t) => t,
            Err(e) => {
                println!("error gettin cache {:?}", e);
                Config {
                    backup_path: format!("C:/Users/{}/Documents/", whoami::username()),
                    mp3_path: String::from("C:/"),
                    jpg_path: String::from("C:/"),
                    static_increment: 1,
                    static_reduction: 1,
                    media_path: String::from("C:/"),
                }
            }
        };
        (
            App {
                is_paused: true,
                current_song: Arc::new(ArcSwap::from_pointee(MusicData::default())),
                sender: None,
                volume: (player_cache.volume * 80.0) as u8, // 80 is out magic number from sink volume -> slider
                shuffle: player_cache.shuffle,
                scrubber: 0,
                time_elapsed: 0,
                total_time: player_cache.length,
                current_view: Page::Main,
                download_page: download_page::DownloadPage::new(),
                setting_page: setting_page::SettingPage::new(&config_cache),
                media_page: crate::gui::media_page::MediaPage::new(&config_cache),
                playlist_page: crate::gui::new_playlist_page::PlaylistPage::new(None),
                song_edit_page: crate::gui::song_edit_page::SongEditPage::new(),
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
            Self::Message::UpdateSender(sender) => {
                println!("updated sender!");
                self.sender = sender;
                Command::none()
            }
            Self::Message::NewData(data) => {
                self.total_time = data.length;
                println!(
                    "The new information given to update: {} {} {}",
                    data.author, data.title, data.album
                );
                self.current_song.store(Arc::new(data));
                Command::none()
            }
            Self::Message::VolumeChange(val) => {
                self.volume = val;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::NewVolume(val))
                    .expect("failure sending msg");
                Command::none()
            }
            Self::Message::MoveSlider(num) => {
                self.scrubber = num;
                // change self.time_elapsed so it makes sense... might be too laggy to calc
                Command::none()
            }
            Self::Message::SkipToSeconds(num) => {
                println!("lets skip to: {}, len: {}", num, self.total_time);
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::SkipToSeconds(num))
                    .unwrap();
                Command::none()
            }
            Self::Message::StaticVolumeUp => {
                // should we try to limit this to 30? the slider max value? makes sense
                self.volume = if self.volume == 30 {
                    30
                } else {
                    self.volume + self.setting_page.static_increment.parse::<u8>().unwrap()
                };
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::NewVolume(self.volume))
                    .unwrap();
                Command::none()
            }
            Self::Message::StaticVolumeDown => {
                self.volume = self
                    .volume
                    .saturating_sub(self.setting_page.static_reduction.parse::<u8>().unwrap());
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::NewVolume(self.volume))
                    .unwrap();
                Command::none()
            }
            Self::Message::ShuffleToggle => {
                self.shuffle = !self.shuffle;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::ToggleShuffle)
                    .unwrap();
                Command::none()
            }
            Self::Message::PlayToggle => {
                self.is_paused = !self.is_paused;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::PlayOrPause)
                    .unwrap();
                Command::none()
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
                Command::none()
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
                Command::none()
            }
            Self::Message::ChangePage(page) => {
                self.current_view = page;
                Command::none()
            }
            Self::Message::UpdateDownloadEntry(string) => {
                self.download_page.text = string;
                Command::none()
            }
            Self::Message::Download(link) => {
                // is it a playlist?
                let download = if link.contains("list=") {
                    let mut list_cmd = Vec::new();
                    let mut link_list = Vec::new();
                    let playlist = get_playlist(&link).unwrap();
                    // to guarentee that the order is preserved, we add an empty entry with just the uuid
                    // then, after the downloads have completed, we either update the entry with the data
                    // or remove the entry afterwards if it fails
                    for song in playlist.links {
                        link_list.push(song.clone()[28..].to_string()); // this is the uniqueid
                        self.download_page
                            .download_feedback
                            .push(format!("Download started on {}", &link));
                        self.download_list.push(song.clone());
                        let cmd = Command::perform(
                            download_interface(song.clone(), Some(playlist.title.clone())),
                            |yt_data| ProgramCommands::AddToDownloadFeedback(song, yt_data),
                        );
                        list_cmd.push(cmd);
                    }
                    // add the empty entries!
                    add_empty_entries(link_list).unwrap();
                    Command::batch(list_cmd)
                } else {
                    self.download_list.push(link.clone());
                    self.download_page
                        .download_feedback
                        .push(format!("Download started on {}", &link));
                    Command::perform(download_interface(link.clone(), None), |yt_data| {
                        ProgramCommands::AddToDownloadFeedback(link, yt_data)
                    })
                };

                // reset the value, regardless of the outcome
                self.download_page.text = String::new();
                download
                // Command::none()
            }
            Self::Message::DownloadMedia(link, path, mp3_4) => Command::perform(
                crate::gui::media_page::download_content(link, path, mp3_4),
                ProgramCommands::DownloadMediaWorked,
            ),
            Self::Message::DownloadMediaWorked(maybe) => {
                let val = match maybe {
                    Ok(t) => t,
                    Err(e) => {
                        format!("Error downloading: {:?}", e)
                    }
                };
                self.media_page.download_feedback.push(val);
                Command::none()
            }
            Self::Message::UpdateMp3Or4Combobox(choosen) => {
                self.media_page.download_type = choosen;
                Command::none()
            }

            Self::Message::SearchYouTube(str) => {
                Command::perform(crate::yt::search::content_to_text(str), |vals| {
                    ProgramCommands::SearchYouTubeResults(vals)
                })
            }
            Self::Message::SearchYouTubeResults(search) => {
                self.download_page.youtube_content.extend(search);
                Command::none()
            }

            Self::Message::AddToDownloadFeedback(link, youtubedata) => {
                // remove it from the download list, since it has either been downloaded, or failed to download
                self.download_list.swap_remove(
                    // swap remove is a little quicker, and order doesn't matter :)
                    self.download_list
                        .iter()
                        .position(|x| *x == link)
                        .expect("Failure removing!"),
                );
                let feedback = match youtubedata {
                    Ok(t) => {
                        // if we are listening to main, update the playlist with the song we just added
                        if self.current_song.load().playlist == "main" {
                            self.sender
                                .as_mut()
                                .unwrap()
                                .send(PungeCommand::ChangePlaylist(String::from("main")))
                                .unwrap();
                        }
                        format!("{} - {} Downloaded Successfully", t.title, t.author)
                    }
                    Err(e) => {
                        println!("ERROR DOWNLOADING: {:?} {:?}", e, &link);
                        // if the problem occured from playlist, there is an existing entry for the obj, but if it failed, we want to
                        // remove that, since it will cause a panic on null fields.
                        // so case where the link is less than 11 chars, it will panic on subtract overflow..
                        if link.len() < 12 {
                            println!("ignoring potential delte action, link is too short");
                        } else {
                            match crate::db::update::delete_from_uuid(
                                link[link.len() - 11..].to_string(), // last 11 chars of the url, aka uniqueid
                            ) {
                                Ok(_t) => {
                                    println!("Deleted successfully: {}", &link);
                                }
                                Err(_e) => {
                                    println!("nothin to delete")
                                }
                            };
                        }
                        format!("Error downloading: {}\n{:?}", link, e)
                    }
                };
                self.download_page.download_feedback.push(feedback);

                Command::none()
            }
            Self::Message::Debug => {
                println!("Da list: {:?}", self.download_list);
                Command::none()
            }
            Self::Message::InAppEvent(t) => match t {
                AppEvent::CloseRequested => {
                    let lcl = self.current_song.load();
                    let cache = player_cache::Cache {
                        song_id: lcl.song_id.clone(),
                        volume: lcl.volume,
                        shuffle: lcl.shuffle,
                        playlist: lcl.playlist.clone(),
                        length: 190,
                    };
                    player_cache::dump_cache(cache); // dumps user cache
                    println!("dumpepd cache!");

                    iced::window::close::<Self::Message>(iced::window::Id::MAIN)
                }
            },
            Self::Message::UpdateSearch(input) => {
                self.search = input;
                Command::none()
            }
            Self::Message::SongFound(obj_or_err) => {
                match obj_or_err {
                    Ok(obj) => {
                        self.sender
                            .as_ref()
                            .unwrap()
                            .send(PungeCommand::ChangeSong(obj.uniqueid))
                            .unwrap();
                        self.search = "".to_string();
                    }
                    Err(_e) => self.search = "No results found".to_string(),
                };
                Command::none()
            }

            Self::Message::GoToSong => Command::perform(
                get_values_from_db(
                    self.current_song.load().playlist.clone(),
                    self.search.clone(),
                ),
                ProgramCommands::SongFound,
            ),
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
                Command::none()
            }
            Self::Message::ChangeViewingPlaylist(playlist) => {
                // we will change the current view to the playlist view, and pass in the playlist to fill the content
                self.viewing_playlist = playlist.clone();

                // main should be treated just like a regular playlist !?
                self.refresh_playlist();
                Command::none()
            }
            Self::Message::SelectSong(uniqueid, song) => {
                // when the song is selected from the table, update the song in the top right
                self.side_menu_song_select = (song, uniqueid);
                // maybe buttons should bring title with it??? idk
                Command::none()
            }
            Self::Message::PlaylistSelected(name) => {
                // set playlist uuid string to uniqueid and set side to the title of that uuid
                let uniqueid = get_uuid_from_name(name.clone());
                self.side_menu_playlist_select = (name, uniqueid);
                Command::none()
            }
            Self::Message::AddToPlaylist(song_id, playlist_id) => {
                if song_id.is_none() | playlist_id.is_none() {
                    println!("fail!")
                } else {
                    add_to_playlist(playlist_id.unwrap(), song_id.unwrap()).unwrap(); // what abt duplicate addigs?
                                                                                      // clear the song once it was added :)
                    self.side_menu_song_select = ("".to_string(), "".to_string())
                }
                Command::none()
            }
            Self::Message::DeleteSong(uuid) => {
                if self.viewing_playlist == "main" {
                    match delete_record_and_file(uuid) {
                        Ok(_t) => {
                            println!("epic delete moment")
                        }
                        Err(e) => {
                            println!("error deleting {:?}", e)
                        }
                    }
                } else {
                    delete_from_playlist(uuid, self.viewing_playlist.clone()).unwrap();
                }
                // refresh current playlist
                // should i function this? used twice..
                self.refresh_playlist();
                Command::none()
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
                Command::none()
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
                Command::none()
            }
            Self::Message::UpdateWidgetText(text_type, txt) => match text_type {
                TextType::BackupText => {
                    self.setting_page.backup_text = txt;
                    Command::none()
                }
                TextType::Mp3Text => {
                    self.setting_page.mp3_path_text = txt;
                    Command::none()
                }
                TextType::JpgText => {
                    self.setting_page.jpg_path_text = txt;
                    Command::none()
                }
                TextType::StaticIncrement => {
                    self.setting_page.static_increment = txt;
                    Command::none()
                }
                TextType::StaticReduction => {
                    self.setting_page.static_reduction = txt;
                    Command::none()
                }
                TextType::UserTitle => {
                    self.playlist_page.user_title = txt;
                    Command::none()
                }
                TextType::UserDescription => {
                    self.playlist_page.user_description = txt;
                    Command::none()
                }
                TextType::UserThumbnail => {
                    self.playlist_page.user_thumbnail = txt;
                    Command::none()
                }
                TextType::Mp4DownloadInput => {
                    self.media_page.download_input = txt;
                    Command::none()
                }
                TextType::Mp4PathInput => {
                    self.media_page.download_to_location = txt;
                    Command::none()
                }
                TextType::TitleChange => {
                    self.song_edit_page.title = txt;
                    Command::none()
                }
                TextType::AuthorChange => {
                    self.song_edit_page.author = txt;
                    Command::none()
                }
                TextType::AlbumChange => {
                    self.song_edit_page.album = txt;
                    Command::none()
                }
                TextType::DownloadLinkInput => {
                    self.download_page.text = txt;
                    Command::none()
                }
                TextType::YouTubeSearchInput => {
                    self.download_page.search_text = txt;
                    Command::none()
                }
                TextType::MediaPath => {
                    self.setting_page.media_path = txt;
                    Command::none()
                }
            },

            Self::Message::SaveConfig => {
                let static_increment = self
                    .setting_page
                    .static_increment
                    .clone()
                    .parse::<usize>()
                    .unwrap();
                let static_reduction = self
                    .setting_page
                    .static_reduction
                    .clone()
                    .parse::<usize>()
                    .unwrap();

                let obj = Config {
                    backup_path: self.setting_page.backup_text.clone(),
                    mp3_path: self.setting_page.mp3_path_text.clone(),
                    jpg_path: self.setting_page.jpg_path_text.clone(),
                    static_increment,
                    static_reduction,
                    media_path: self.setting_page.media_path.clone(),
                };
                match cache::write_to_cache(obj) {
                    Ok(_t) => {
                        println!("config written successfully")
                    }
                    Err(e) => {
                        println!("Config failed! {:?}", e)
                    }
                }
                Command::none()
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
                Command::none()
            }
            Self::Message::OpenSongEditPage(uniqueid) => {
                // empty uniqueid will crash program, check against it
                if !uniqueid.is_empty() {
                    let item = song_from_uuid(&uniqueid).unwrap();
                    self.song_edit_page
                        .update_info(item.0, item.1, item.2, uniqueid);
                    self.current_view = Page::SongEdit;
                }
                Command::none()
            }
            Self::Message::UpdateSong(row) => {
                update_song(row.author, row.title, row.album, row.uniqueid).unwrap();
                self.refresh_playlist();
                // update the active playlists in memory with the new name, im not sure if there is a better way
                // to do this, just reload the playlist ig
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::ChangePlaylist(self.viewing_playlist.clone()))
                    .unwrap();
                self.current_view = Page::Main;
                Command::none()
            }

            _ => {
                println!("inumplmented");
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let table = responsive(|_size| {
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
            button(text("Edit!")).on_press(ProgramCommands::OpenSongEditPage(
                self.side_menu_song_select.1.clone()
            )),
            button(text("DELETE!")).on_press(ProgramCommands::DeleteSong(
                self.side_menu_song_select.1.clone()
            )),
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
            vertical_space(),
            self.render_bottom_bar(),
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
            Page::SongEdit => self.song_edit_page.view(),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::subscription::Subscription::batch(vec![
            self.music_loop(),
            self.hotkey_loop(),
            self.database_subscription(self.current_song.clone()),
            self.close_app_sub(),
            self.discord_loop(self.current_song.clone()), // self.database_sub(database_receiver),
        ]) // is two batches required?? prolly not
    }
}

impl App {
    fn refresh_playlist(&mut self) {
        if self.viewing_playlist == "main" {
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
            let new = get_all_from_playlist(&self.viewing_playlist).unwrap();
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
        // if we are listening to main, the playlist refreshes because of a download, update the main playlist in place
    }
}
