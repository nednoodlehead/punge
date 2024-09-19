use crate::db::fetch::{get_all_from_playlist, get_all_main, get_all_playlists, get_obj_from_uuid};
use crate::db::insert::create_playlist;
use crate::db::update::{
    delete_from_playlist, delete_playlist, move_song_down_one, move_song_up_one,
    quick_swap_title_author, update_auth_album, update_song,
};
use crate::gui::messages::{
    AppEvent, CheckBoxType, ComboBoxType, Context, Page, ProgramCommands, PungeCommand, TextType,
};
use crate::gui::{download_page, setting_page};
use crate::player::player_cache;
use crate::player::sort::get_values_from_db;
use crate::types::{AppError, Config, MusicData, ShuffleType, UserPlaylist};
use crate::utils::backup::create_backup;
use crate::utils::cache;
use crate::utils::delete::delete_record_and_file;
use crate::yt::interface::download_interface;
use arc_swap::ArcSwap;
use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager};
use iced::subscription::Subscription;
use iced::widget::{column, container, image, row, scrollable, text};
use iced::{Command, Element, Length, Settings, Theme};
use log::{debug, error, info, warn};
use simplelog::{CombinedLogger, TermLogger, WriteLogger};
use std::sync::Arc;
use tokio::sync::mpsc as async_sender; // does it need to be in scope?

pub fn begin() -> iced::Result {
    // initialze logger
    let mut log_config = simplelog::ConfigBuilder::new();
    log_config.add_filter_allow("punge".to_string());
    CombinedLogger::init(vec![
        TermLogger::new(
            log::LevelFilter::Warn,
            log_config.build(),
            simplelog::TerminalMode::default(),
            simplelog::ColorChoice::Always,
        ),
        WriteLogger::new(
            log::LevelFilter::Debug,
            log_config.build(),
            std::fs::File::create("punge.log").unwrap(),
        ),
    ])
    .unwrap();
    iced::program("Punge!!", App::update, App::view)
        .settings(Settings {
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
                icon: Some(iced::window::icon::from_file("./img/punge icon.ico").unwrap()),
                platform_specific: iced::window::settings::PlatformSpecific::default(),
                exit_on_close_request: false,
            },
            default_font: iced::Font::with_name("Punge!!"),
            default_text_size: iced::Pixels(16.0),
            antialiasing: false,
            fonts: vec![std::borrow::Cow::Borrowed(iced_aw::BOOTSTRAP_FONT_BYTES)], // thanks source code?
        })
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}
// pages for the gui
pub struct App {
    pub is_paused: bool,
    pub current_song: Arc<ArcSwap<MusicData>>, // represents title, auth, album, song_id, volume, shuffle, playlist
    sender: Option<async_sender::UnboundedSender<PungeCommand>>, // was not an option before !
    pub volume: u8,
    pub shuffle: bool,
    pub scrubber: u32,
    pub silence_scrubber: bool,            // when we start dragging
    pub time_elapsed: std::time::Duration, // needs to be a u32 im pretty sure
    pub total_time: u32,                   // not u64 because u cant go from u64 -> f64? ig
    current_view: Page,
    download_page: crate::gui::download_page::DownloadPage,
    pub setting_page: setting_page::SettingPage, // pub so src\gui\subscrip can see the user choosen value increments
    media_page: crate::gui::media_page::MediaPage,
    playlist_page: crate::gui::new_playlist_page::PlaylistPage,
    song_edit_page: crate::gui::song_edit_page::SongEditPage,
    download_list: Vec<String>, // full link, songs are removed when finished / errored. Used so multiple downloads are not started
    manager: GlobalHotKeyManager, // TODO at some point: make interface for re-binding
    pub config: Arc<ArcSwap<Config>>, // also contains hotkeys :D
    pub search: String,
    viewing_playlist: String, // could derive from cache soon... just the uniqueid rn
    selected_songs: Vec<(Option<usize>, String)>, // songs that the user will edit. if is_some, unselect the rows in the table
    pub user_playlists: Vec<UserPlaylist>,
    // tarkah table stuff
    table_content: iced::widget::list::Content<crate::gui::widgets::row::RowData>,
}

impl Default for App {
    fn default() -> Self {
        let manager = GlobalHotKeyManager::new().unwrap();
        let player_cache = player_cache::fetch_cache();
        let config_cache = match cache::read_from_cache() {
            Ok(t) => {
                // what abt no mods? maybe should check
                for (_, bind) in t.keybinds.iter() {
                    let hotkey = if bind.mod1.is_none() {
                        HotKey::new(bind.mod2, bind.code.unwrap())
                    } else if bind.mod2.is_none() {
                        HotKey::new(bind.mod1, bind.code.unwrap())
                    } else {
                        HotKey::new(
                            Some(bind.mod1.unwrap() | bind.mod2.unwrap()),
                            bind.code.unwrap(),
                        )
                    };
                    manager.register(hotkey).unwrap();
                }
                t
            }
            Err(_) => {
                warn!("Cannot fetch cache, resorting to default");
                Config {
                    backup_path: format!("C:/Users/{}/Documents/", whoami::username()),
                    mp3_path: String::from("C:/"),
                    jpg_path: String::from("C:/"),
                    static_increment: 1,
                    static_reduction: 1,
                    media_path: String::from("C:/"),
                    keybinds: std::collections::HashMap::new(), // empty!
                    shuffle_type: ShuffleType::Regular,
                    idle_strings: vec!["listening to nothin".to_string()],
                }
            }
        };
        App {
            is_paused: true,
            current_song: Arc::new(ArcSwap::from_pointee(MusicData::default())),
            sender: None,
            volume: (player_cache.volume * 80.0) as u8, // 80 is out magic number from sink volume -> slider
            shuffle: player_cache.shuffle,
            scrubber: 0,
            silence_scrubber: false,
            time_elapsed: std::time::Duration::default(),
            total_time: player_cache.length,
            current_view: Page::Main,
            download_page: download_page::DownloadPage::new(),
            setting_page: setting_page::SettingPage::new(&config_cache),
            media_page: crate::gui::media_page::MediaPage::new(&config_cache),
            playlist_page: crate::gui::new_playlist_page::PlaylistPage::new(None),
            song_edit_page: crate::gui::song_edit_page::SongEditPage::new(),
            download_list: vec![],
            manager,
            config: Arc::new(ArcSwap::from_pointee(config_cache)),
            search: "".to_string(),
            viewing_playlist: "main".to_string(),
            selected_songs: vec![],
            user_playlists: get_all_playlists().unwrap(), // im addicted to unwraping
            // maybe do most recent playlist next? from cache?
            table_content: get_all_main()
                .unwrap()
                .into_iter()
                .enumerate()
                .map(|(count, item)| crate::gui::widgets::row::RowData {
                    title: item.title.clone(),
                    author: item.author.clone(),
                    album: item.album.clone(),
                    row_num: count,
                    uniqueid: item.uniqueid.clone(),
                })
                .collect(),
        }
    }
}

impl App {
    fn theme(&self) -> Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, msg: ProgramCommands) -> iced::Command<ProgramCommands> {
        match msg {
            ProgramCommands::UpdateSender(sender) => {
                info!("Sender sent!");
                self.sender = sender;
                Command::none()
            }
            ProgramCommands::NewData(data) => {
                self.total_time = data.length;
                match &data.context {
                    &Context::Default => {}
                    &Context::PlayPause => {}
                    &Context::SkippedTo => {}

                    _ => {
                        info!("resetting scrubber to 0");
                        self.scrubber = 0;
                    }
                }
                info!(
                    "The new information given to update: {} {} {}",
                    data.author, data.title, data.album
                );
                self.current_song.store(Arc::new(data));
                Command::none()
            }
            ProgramCommands::VolumeChange(val) => {
                self.volume = val;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::NewVolume(val))
                    .expect("failure sending msg");
                Command::none()
            }
            ProgramCommands::MoveSlider(val) => {
                // when this is called, we should silence any new information that the automatic change does
                self.silence_scrubber = true;
                self.scrubber = val;
                // maybe in the future, we should have a function about here that tells the user what time they are skipping to
                // something like ( ( self.total_duration * 10 - val ) / 10 ) -> converted into MM:SS ??
                Command::none()
            }
            ProgramCommands::SkipToSeconds(num) => {
                // this command will release the silence on the automatic updates for the scrubbing bar
                info!("lets skip to: {}, len: {}", num, self.total_time);
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::SkipToSeconds(num))
                    .unwrap();
                self.silence_scrubber = false;
                Command::none()
            }
            ProgramCommands::StaticVolumeUp => {
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
            ProgramCommands::StaticVolumeDown => {
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
            ProgramCommands::ShuffleToggle => {
                self.shuffle = !self.shuffle;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::ToggleShuffle)
                    .unwrap();
                Command::none()
            }
            ProgramCommands::PlayToggle => {
                self.is_paused = !self.is_paused;
                self.sender
                    .as_mut()
                    .unwrap()
                    .send(PungeCommand::PlayOrPause)
                    .unwrap();
                Command::none()
            }
            ProgramCommands::SkipForwards => {
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
            ProgramCommands::SkipBackwards => {
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
            ProgramCommands::ChangePage(page) => {
                self.current_view = page;
                Command::none()
            }
            ProgramCommands::Download(link) => {
                // we need to find the current length for main
                let download = if link.contains("list=") {
                    Command::perform(
                        crate::yt::interface::playlist_wrapper(link.clone()),
                        |playl| {
                            let bew = if playl.is_err() {
                                Err(AppError::YoutubeError(format!("{:?}", playl)))
                            } else {
                                Ok(playl.unwrap())
                            };

                            ProgramCommands::PlaylistResults(link, bew)
                        },
                    )
                } else {
                    self.download_list.push(link.clone());
                    self.download_page
                        .download_feedback
                        .push(format!("Download started on {}", &link));
                    Command::perform(
                        download_interface(link.clone(), None, self.user_playlists[0].songcount),
                        |yt_data| ProgramCommands::AddToDownloadFeedback(link, yt_data),
                    )
                };

                // reset the value, regardless of the outcome
                self.download_page.text = String::new();
                download
                // Command::none()
            }
            ProgramCommands::PlaylistResults(link, playlist_or_err) => {
                if playlist_or_err.is_err() {
                    self.download_page
                        .download_feedback
                        .push(format!("Download failed!: {}", link));
                    return Command::none();
                }
                let playlist = playlist_or_err.unwrap();
                let mut list_cmd = Vec::new();
                // to guarentee that the order is preserved, we add an empty entry with just the uuid
                // then, after the downloads have completed, we either update the entry with the data
                // or remove the entry afterwards if it fails
                let mut count = 0;
                let default_count = self.user_playlists[0].songcount;
                for song in playlist.videos.clone() {
                    let full_url = format!("https://youtube.com/watch?v={}", &song.url);
                    self.download_page
                        .download_feedback
                        .push(format!("Download started on {}", &full_url));
                    self.download_list.push(song.title.clone());
                    let cmd = Command::perform(
                        download_interface(
                            full_url,
                            Some(playlist.name.clone()),
                            default_count + count,
                        ),
                        |yt_data| ProgramCommands::AddToDownloadFeedback(song.title, yt_data),
                    );
                    list_cmd.push(cmd);
                    count += 1;
                }
                // add the empty entries!
                Command::batch(list_cmd)
            }
            ProgramCommands::DownloadMedia(link, path, mp3_4) => {
                self.media_page
                    .download_feedback
                    .push(format!("Starting download on {}", &link));
                self.media_page.download_input = "".to_string();
                Command::perform(
                    crate::gui::media_page::download_content(link, path, mp3_4),
                    ProgramCommands::DownloadMediaWorked,
                )
            }
            ProgramCommands::DownloadMediaWorked(maybe) => {
                let val = match maybe {
                    Ok(t) => t,
                    Err(e) => {
                        format!("Error downloading: {:?}", e)
                    }
                };
                self.media_page.download_feedback.push(val);
                Command::none()
            }
            ProgramCommands::SearchYouTube(str) => {
                // should *in theory* get rid of the images in memory so there is no problem deleteing them from the
                // content_to_text() call (remove_all_in_temp_dir)
                self.download_page.youtube_content = vec![];
                Command::perform(
                    crate::yt::search::content_to_text(
                        str,
                        self.download_page.include_videos,
                        self.download_page.include_playlists,
                    ),
                    ProgramCommands::SearchYouTubeResults,
                )
            }
            ProgramCommands::SearchYouTubeResults(search) => {
                self.download_page.youtube_content = search;
                Command::none()
            }

            ProgramCommands::AddToDownloadFeedback(link, youtubedata) => {
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
                        error!("ERROR DOWNLOADING: {:?} {:?}", e, &link);
                        // if the problem occured from playlist, there is an existing entry for the obj, but if it failed, we want to
                        // remove that, since it will cause a panic on null fields.
                        // so case where the link is less than 11 chars, it will panic on subtract overflow..
                        if link.len() < 12 {
                            warn!("ignoring potential delte action, link is too short");
                        } else {
                            match crate::db::update::delete_from_uuid(
                                link[link.len() - 11..].to_string(), // last 11 chars of the url, aka uniqueid
                            ) {
                                Ok(_t) => {
                                    info!("Deleted successfully: {}", &link);
                                }
                                Err(_e) => {
                                    info!("nothin to delete")
                                }
                            };
                        }
                        format!("Error downloading: {}\n{:?}", link, e)
                    }
                };
                self.download_page.download_feedback.push(feedback);

                Command::none()
            }
            ProgramCommands::Debug => Command::none(),
            ProgramCommands::InAppEvent(t) => match t {
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
                    info!("dumpepd cache! goodbye :)");

                    iced::window::close::<ProgramCommands>(iced::window::Id::MAIN)
                }
            },
            ProgramCommands::UpdateSearch(input) => {
                self.search = input;
                Command::none()
            }
            ProgramCommands::SongFound(obj_or_err) => {
                match obj_or_err {
                    Ok(obj) => {
                        if self.is_paused {
                            self.is_paused = false;
                        }
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

            ProgramCommands::GoToSong => Command::perform(
                get_values_from_db(
                    self.current_song.load().playlist.clone(),
                    self.search.clone(),
                ),
                ProgramCommands::SongFound,
            ),
            ProgramCommands::PlaySong(song) => {
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
                // reset scrubber on successful song change!
                self.scrubber = 0;
                Command::none()
            }
            ProgramCommands::ChangeViewingPlaylist(playlist) => {
                // we will change the current view to the playlist view, and pass in the playlist to fill the content
                self.current_view = Page::Main;
                self.viewing_playlist = playlist.clone();
                self.selected_songs.clear(); // clear them! (so we dont select some, switch playlist and edit unintentionally)
                                             // main should be treated just like a regular playlist !?
                self.refresh_playlist();
                // debug!( i currently have 154 errors in console, im fixin things one at a time
                //     "rows? {} | {:?} name: {}",
                //     self.rows.len(),
                //     self.rows,
                //     &playlist
                // );
                Command::none()
            }
            ProgramCommands::AddToPlaylist(playlist_id, song_id) => {
                let local_songcount = self.user_playlists[self
                    .user_playlists
                    .iter()
                    .position(|play| &play.uniqueid == &playlist_id)
                    .unwrap()]
                .songcount;
                info!(
                    "we will add: {:?} to {}",
                    &self.selected_songs, &playlist_id
                );
                // TODO
                crate::db::insert::add_to_playlist(&playlist_id, &song_id, local_songcount)
                    .unwrap();
                Command::none()
            }
            ProgramCommands::DeleteSong(uuid) => {
                // should ask user if they are sure ?
                // TODO rewrite for the new interface
                if self.viewing_playlist == "main" {
                    match delete_record_and_file(uuid.clone()) {
                        Ok(t) => {
                            info!("Success removing {:?} from main", t);
                        }
                        Err(e) => {
                            error!("Could not remove from main: {:?}", e);
                        }
                    }
                } else {
                    match delete_from_playlist(uuid.clone(), self.viewing_playlist.clone()) {
                        Ok(t) => {
                            info!("Success removing {:?} from playlist", t)
                        }
                        Err(e) => {
                            error!("Could not remove from playlist! {:?}", e)
                        }
                    }
                }
                self.refresh_playlist();
                Command::none()
            }
            ProgramCommands::CreateBackup => {
                // get backup path from config and use it :)

                match create_backup(self.setting_page.backup_text.clone()) {
                    Ok(_) => {
                        info!("backup created successfully!");
                    }
                    Err(e) => {
                        error!("error creating backup -> {:?}", e);
                    }
                };
                Command::none()
            }
            ProgramCommands::UpdateWidgetText(text_type, txt) => match text_type {
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
            ProgramCommands::CheckBoxEvent(checkbox, val) => match checkbox {
                CheckBoxType::IncludeVideos => {
                    self.download_page.include_videos = val;
                    Command::none()
                }
                CheckBoxType::IncludePlaylists => {
                    self.download_page.include_playlists = val;
                    Command::none()
                }
            },
            ProgramCommands::UpdateCombobox(boxtype, txt) => {
                // is there any merit in making a hashmap and matching?
                match boxtype {
                    ComboBoxType::PlayKey => {
                        self.setting_page.play_key_value = txt;
                    }
                    ComboBoxType::PlayModifier1 => {
                        self.setting_page.play_mod1_value = txt;
                    }
                    ComboBoxType::PlayModifier2 => {
                        self.setting_page.play_mod2_value = txt;
                    }
                    ComboBoxType::ForwardKey => {
                        self.setting_page.forward_key_value = txt;
                    }
                    ComboBoxType::ForwardModifer1 => {
                        self.setting_page.forward_mod1_value = txt;
                    }
                    ComboBoxType::ForwardModifer2 => {
                        self.setting_page.forward_mod2_value = txt;
                    }
                    ComboBoxType::BackwardKey => {
                        self.setting_page.backward_key_value = txt;
                    }
                    ComboBoxType::BackwardModifier1 => {
                        self.setting_page.backward_mod1_value = txt;
                    }
                    ComboBoxType::BackwardModifier2 => {
                        self.setting_page.backward_mod2_value = txt;
                    }
                    ComboBoxType::ShuffleKey => {
                        self.setting_page.shuffle_key_value = txt;
                    }
                    ComboBoxType::ShuffleModifier1 => {
                        self.setting_page.shuffle_mod1_value = txt;
                    }
                    ComboBoxType::ShuffleModifier2 => {
                        self.setting_page.shuffle_mod2_value = txt;
                    }
                    ComboBoxType::StaticUpKey => {
                        self.setting_page.staticup_key_value = txt;
                    }
                    ComboBoxType::StaticUpModifier1 => {
                        self.setting_page.staticup_mod1_value = txt;
                    }
                    ComboBoxType::StaticUpModifier2 => {
                        self.setting_page.staticup_mod2_value = txt;
                    }
                    ComboBoxType::StaticDownKey => {
                        self.setting_page.staticdown_key_value = txt;
                    }
                    ComboBoxType::StaticDownModifier1 => {
                        self.setting_page.staticdown_mod1_value = txt;
                    }
                    ComboBoxType::StaticDownModifier2 => {
                        self.setting_page.staticdown_mod2_value = txt;
                    }
                    ComboBoxType::GoToAlbumKey => {
                        self.setting_page.gotoalbum_key_value = txt;
                    }
                    ComboBoxType::GoToAlbumModifier1 => {
                        self.setting_page.gotoalbum_mod1_value = txt;
                    }
                    ComboBoxType::GoToAlbumModifer2 => {
                        self.setting_page.gotoalbum_mod2_value = txt;
                    }
                    ComboBoxType::Mp3Or4 => {
                        self.media_page.download_type = txt;
                    }
                    ComboBoxType::ShuffleType => {
                        self.setting_page.shuffle_type = txt;
                    }
                }
                Command::none()
            }

            ProgramCommands::SaveConfig => {
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
                let binds = vec![
                    (
                        self.setting_page.play_key_value.clone(),
                        self.setting_page.play_mod1_value.clone(),
                        self.setting_page.play_mod2_value.clone(),
                    ),
                    (
                        self.setting_page.forward_key_value.clone(),
                        self.setting_page.forward_mod1_value.clone(),
                        self.setting_page.forward_mod2_value.clone(),
                    ),
                    (
                        self.setting_page.backward_key_value.clone(),
                        self.setting_page.backward_mod1_value.clone(),
                        self.setting_page.backward_mod2_value.clone(),
                    ),
                    (
                        self.setting_page.shuffle_key_value.clone(),
                        self.setting_page.shuffle_mod1_value.clone(),
                        self.setting_page.shuffle_mod2_value.clone(),
                    ),
                    (
                        self.setting_page.staticup_key_value.clone(),
                        self.setting_page.staticup_mod1_value.clone(),
                        self.setting_page.staticup_mod2_value.clone(),
                    ),
                    (
                        self.setting_page.staticdown_key_value.clone(),
                        self.setting_page.staticdown_mod1_value.clone(),
                        self.setting_page.staticdown_mod2_value.clone(),
                    ),
                    (
                        self.setting_page.gotoalbum_key_value.clone(),
                        self.setting_page.gotoalbum_mod1_value.clone(),
                        self.setting_page.gotoalbum_mod2_value.clone(),
                    ),
                ];
                let curr_hotkeys = self
                    .config
                    .load()
                    .keybinds
                    .iter()
                    .map(|key| {
                        if key.1.mod1.is_none() {
                            HotKey::new(key.1.mod2, key.1.code.unwrap())
                        } else if key.1.mod2.is_none() {
                            HotKey::new(key.1.mod1, key.1.code.unwrap())
                        } else {
                            HotKey::new(
                                Some(key.1.mod1.unwrap() | key.1.mod2.unwrap()),
                                key.1.code.unwrap(),
                            )
                        }
                    })
                    .collect::<Vec<HotKey>>();
                // get all current hotkeys in config and unbind them
                self.manager.unregister_all(&curr_hotkeys).unwrap();
                // this is the order that the bind types are collected
                let cmd_order = vec![
                    ProgramCommands::PlayToggle,
                    ProgramCommands::SkipForwards,
                    ProgramCommands::SkipBackwards,
                    ProgramCommands::ShuffleToggle,
                    ProgramCommands::StaticVolumeUp,
                    ProgramCommands::StaticVolumeDown,
                    ProgramCommands::GoToSong,
                ];
                // will hold the final config
                let mut bind_config = std::collections::HashMap::from([]);
                // loop through our potential binds
                for (count, (key, va1, va2)) in binds.iter().enumerate() {
                    // if the key isn't nothing, lets do something
                    if key != &"" {
                        let val = crate::gui::setting_page::strings_to_hashmap(
                            key.clone(),
                            va1.clone(),
                            va2.clone(),
                            cmd_order[count].clone(),
                        );
                        bind_config.insert(val.0, val.1.clone());
                        let newkey = if val.1.mod1.is_none() {
                            HotKey::new(val.1.mod2, val.1.code.unwrap())
                        } else if val.1.mod2.is_none() {
                            HotKey::new(val.1.mod1, val.1.code.unwrap())
                        } else {
                            HotKey::new(
                                Some(val.1.mod1.unwrap() | val.1.mod2.unwrap()),
                                val.1.code.unwrap(),
                            )
                        };
                        self.manager.register(newkey).unwrap();
                    }
                }

                let obj = Config {
                    backup_path: self.setting_page.backup_text.clone(),
                    mp3_path: self.setting_page.mp3_path_text.clone(),
                    jpg_path: self.setting_page.jpg_path_text.clone(),
                    static_increment,
                    static_reduction,
                    media_path: self.setting_page.media_path.clone(),
                    keybinds: bind_config,
                    shuffle_type: ShuffleType::from_str(&self.setting_page.shuffle_type),
                    idle_strings: crate::gui::setting_page::idle_strings_to_config(
                        self.setting_page.idle_string_content.text(),
                    ),
                };
                // mostly useful for updating keybinds in real time
                self.config.store(Arc::new(obj.clone())); // refresh the config with this data :D
                match cache::write_to_cache(obj) {
                    Ok(_t) => {
                        info!("config written successfully: {:?}", &self.config.load())
                    }
                    Err(e) => {
                        warn!("Config failed! {:?}", e)
                    }
                }
                // update the path when the user is sure of the default location
                self.media_page.download_to_location = self.setting_page.media_path.clone();
                Command::none()
            }
            ProgramCommands::NewPlaylist => {
                if !self.playlist_page.user_title.is_empty() {
                    // check to see if it is empty..
                    let playlist = UserPlaylist::new(
                        self.playlist_page.user_title.clone(),
                        self.playlist_page.user_description.clone(),
                        self.playlist_page.user_thumbnail.clone(),
                        false,
                    );
                    create_playlist(playlist).unwrap();
                    self.user_playlists = get_all_playlists().unwrap();
                    // also refresh the buttons!
                    self.playlist_page.user_title.clear();
                    self.playlist_page.user_description.clear();
                    self.playlist_page.user_thumbnail.clear();
                    self.current_view = Page::Main;
                }
                Command::none()
            }
            ProgramCommands::DeletePlaylist(uuid) => {
                // maybe we can "mark" a playlist for deletion? so the user has to click it twice?
                // a little confirmation menu would be best. but i dont think there is support for that?
                crate::db::update::delete_playlist(&uuid).unwrap();
                Command::none()
            }

            ProgramCommands::DuplicatePlaylist(uuid) => {
                crate::db::update::duplicate_playlist(&uuid).unwrap();
                Command::none()
            }

            ProgramCommands::OpenSongEditPage(maybe_string) => {
                let data = if maybe_string.is_none() {
                    let info = self.current_song.load();
                    (
                        info.title.clone(),
                        info.author.clone(),
                        info.album.clone(),
                        info.song_id.clone(),
                    )
                } else {
                    let info = get_obj_from_uuid(&maybe_string.unwrap()).unwrap(); // no real guarentee that this is the right one
                                                                                   // since we use remove_swap...
                    (info.title, info.author, info.album, info.uniqueid)
                };
                self.song_edit_page.update_info(
                    data.0,
                    data.1,
                    data.2,
                    data.3,
                    false,
                    self.selected_songs.len() > 1, // are multiple songs selected?
                );
                self.current_view = Page::SongEdit;
                Command::none()
            }
            ProgramCommands::UpdateSong(row) => {
                if self.song_edit_page.multi_select {
                    // if multiple songs are selected
                    for id in self.selected_songs.iter() {
                        update_auth_album(row.author.clone(), row.album.clone(), id.1.to_string())
                            .unwrap();
                    }
                } else {
                    update_song(row.author, row.title, row.album, row.uniqueid).unwrap();
                }
                // i dont think there is a way to
                self.refresh_playlist();
                self.selected_songs.clear();

                self.current_view = Page::Main;
                Command::none()
            }
            ProgramCommands::QuickSwapTitleAuthor(uuid_to_swap) => {
                if self.song_edit_page.multi_select {
                    for id in self.selected_songs.iter() {
                        info!("swapping multiple uuids: {}", &id.1);
                        quick_swap_title_author(&id.1).unwrap();
                    }
                } else {
                    info!("swapping a single uuid");
                    quick_swap_title_author(&uuid_to_swap).unwrap();
                }
                self.refresh_playlist();
                Command::none()
            }
            ProgramCommands::PushScrubber(duration) => {
                // we need to figure out two things:
                // what the current duration elapsed is (put it from 110s -> 1:30)
                // where the scrubber bar should be (total steps = len * 10)
                self.time_elapsed = duration;
                if !self.silence_scrubber {
                    self.scrubber = (duration.as_millis() / 100) as u32;
                }
                // self.scrubber = new as u32;

                Command::none()
            }
            ProgramCommands::DeletePlaylist(id) => {
                // you cannot delete main.
                if id.to_lowercase() != "main" {
                    delete_playlist(&id).unwrap();
                    // refresh the playlist...
                    self.user_playlists = get_all_playlists().unwrap();
                }
                Command::none()
            }
            ProgramCommands::UpdatePlaylist => {
                crate::db::update::update_playlist(
                    &self.playlist_page.user_title,
                    &self.playlist_page.user_description,
                    &self.playlist_page.user_thumbnail,
                    self.playlist_page.user_id.clone().unwrap().as_ref(), // unwrap because the button that calls this is conditional on self.user_id.is_some()
                )
                .unwrap();
                self.user_playlists = get_all_playlists().unwrap();
                self.playlist_page.user_title = "".to_string();
                self.playlist_page.user_description = "".to_string();
                self.playlist_page.user_thumbnail = "".to_string();
                self.playlist_page.user_id = None;
                self.current_view = Page::Main;
                Command::none()
            }
            ProgramCommands::OpenPlaylistEditPage(playlist) => {
                self.current_view = Page::Playlist;
                self.playlist_page.user_title = playlist.title;
                self.playlist_page.user_description = playlist.description;
                self.playlist_page.user_thumbnail = playlist.thumbnail;
                self.playlist_page.user_id = Some(playlist.uniqueid);
                Command::none()
            }
            ProgramCommands::ClearPlaylistPage => {
                self.playlist_page.user_title = "".to_string();
                self.playlist_page.user_description = "".to_string();
                self.playlist_page.user_thumbnail = "".to_string();
                self.playlist_page.user_id = None;
                // put user back to home screen
                self.current_view = Page::Main;
                Command::none()
            }
            ProgramCommands::MovePlaylistUp(uniqueid) => {
                println!("Goin up!!");
                crate::db::update::move_playlist_up_one(&uniqueid).unwrap();
                self.user_playlists = get_all_playlists().unwrap();
                Command::none()
            }
            ProgramCommands::MovePlaylistDown(uniqueid) => {
                println!("GOING DOWN");
                crate::db::update::move_playlist_down_one(&uniqueid).unwrap();
                self.user_playlists = get_all_playlists().unwrap();
                Command::none()
            }
            ProgramCommands::MoveSongUp(uuid, position) => {
                if position != 0 {
                    move_song_up_one(uuid, position, self.viewing_playlist.clone()).unwrap();
                    self.refresh_playlist();
                } else {
                    warn!("MoveSongUp called on song in position 0!")
                }
                Command::none()
            }
            ProgramCommands::MoveSongDown(uuid, position) => {
                if position.saturating_sub(1) != self.table_content.len() {
                    move_song_down_one(uuid, position, self.viewing_playlist.clone()).unwrap();
                    self.refresh_playlist();
                } else {
                    warn!("MoveSongDown called on lowest song")
                }
                Command::none()
            }
            ProgramCommands::UpdateEditor(action) => {
                self.setting_page.idle_string_content.perform(action);
                Command::none()
            }
            ProgramCommands::SelectSong(row_num, is_selected, uuid) => {
                println!("{} is selected: {}", row_num, is_selected);
                if is_selected {
                    self.selected_songs.push((Some(row_num), uuid));
                } else {
                    self.selected_songs.swap_remove(
                        self.selected_songs
                            .iter()
                            .position(|item| item.1 == uuid)
                            .unwrap(),
                    );
                }
                println!("{:#?}", &self.selected_songs);
                Command::none()
            }

            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, ProgramCommands> {
        let table = scrollable(iced::widget::list(&self.table_content, |index, item| {
            crate::gui::widgets::row::RowWidget::new(
                &item.title,
                &item.author,
                &item.album,
                index,
                ProgramCommands::DeleteSong,
                ProgramCommands::QuickSwapTitleAuthor, // needs updating..
                ProgramCommands::SelectSong,
                ProgramCommands::AddToPlaylist,
                ProgramCommands::PlaySong,
                ProgramCommands::MoveSongUp,
                ProgramCommands::MoveSongDown,
                ProgramCommands::OpenSongEditPage,
                self.user_playlists
                    .iter()
                    .map(|playl| (playl.uniqueid.clone(), playl.title.clone()))
                    .collect(),
                item.uniqueid.clone(),
            )
            .into()
        }))
        .width(1000);

        let mut all_playlists_but_main = self.user_playlists.clone();
        all_playlists_but_main.remove(0);
        // user should always have the 'main' playlist
        let active_playlist = self.user_playlists[self
            .user_playlists
            .iter()
            .position(|x| x.uniqueid == self.viewing_playlist)
            .unwrap_or(0)]
        .clone();

        let table_cont = container(table).height(Length::Fill).padding(5);
        let table_cont_wrapper = column![
            row![
                // playlist data
                image::Image::new(active_playlist.thumbnail)
                    .width(90)
                    .height(90),
                text(active_playlist.title).size(35),
                text(active_playlist.description),
            ]
            .padding(5)
            .align_items(iced::Alignment::End)
            .spacing(25),
            table_cont
        ];

        let main_page_2 = container(column![
            row![self.render_buttons_side(Page::Main), table_cont_wrapper],
            self.render_bottom_bar()
        ]);
        match self.current_view {
            // which page to display
            Page::Main => main_page_2.into(),
            Page::Download => column![
                row![
                    self.render_buttons_side(Page::Download),
                    self.download_page.view(),
                ],
                self.render_bottom_bar(),
            ]
            .into(),
            Page::Settings => column![
                row![
                    self.render_buttons_side(Page::Settings),
                    self.setting_page.view(),
                ],
                self.render_bottom_bar(),
            ]
            .into(),
            Page::Media => column![
                row![
                    self.render_buttons_side(Page::Media),
                    self.media_page.view()
                ],
                self.render_bottom_bar(),
            ]
            .into(),
            Page::Playlist => column![
                row![
                    self.render_buttons_side(Page::Playlist),
                    self.playlist_page.view(),
                ],
                self.render_bottom_bar(),
            ]
            .into(),
            Page::SongEdit => column![
                row![
                    self.render_buttons_side(Page::SongEdit),
                    self.song_edit_page.view()
                ],
                self.render_bottom_bar(),
            ]
            .into(),
        }
    }

    fn subscription(&self) -> Subscription<ProgramCommands> {
        iced::subscription::Subscription::batch(vec![
            self.music_loop(self.config.clone()),
            self.hotkey_loop(self.config.clone()),
            self.database_subscription(self.current_song.clone()),
            self.close_app_sub(),
            self.discord_loop(self.current_song.clone(), self.config.clone()),
            // need to just be able to read the memory. aaaaaaahhh
        ]) // is two batches required?? prolly not
    }
}

impl App {
    fn refresh_playlist(&mut self) {
        // are we able to replace .enumerate() with item.count? <-- when it is consistent!!
        if self.viewing_playlist.to_lowercase() == "main" {
            let new = get_all_main().unwrap();
            self.table_content = new
                .into_iter()
                .enumerate()
                .map(|(count, item)| crate::gui::widgets::row::RowData {
                    title: item.title,
                    author: item.author,
                    album: item.album,
                    uniqueid: item.uniqueid,
                    row_num: count,
                })
                .collect();
        } else {
            let new = get_all_from_playlist(&self.viewing_playlist).unwrap();
            debug!("viewing_playlist: {:?}", &self.viewing_playlist);
            self.table_content = new
                .into_iter()
                .enumerate()
                .map(|(count, item)| crate::gui::widgets::row::RowData {
                    title: item.title,
                    author: item.author,
                    album: item.album,
                    uniqueid: item.uniqueid,
                    row_num: count,
                })
                .collect();
        }
        // if we are listening to main, the playlist refreshes because of a download, update the main playlist in place
    }
}
