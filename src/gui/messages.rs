// these are the messages sent around the program. This is divded up here because start.rs imports this
// as well as the actual music-playing portion of the app

use crate::types::{AppError, PungeMusicObject, UserPlaylist};
use crate::types::{MusicData, YouTubeData};
use serde::{ser, Deserialize, Serialize};
use tokio::sync::mpsc as async_sender;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PungeCommand {
    PlayOrPause,
    ChangeSong(String), // play this song's uuid, loop will find the index and swap to it
    NewVolume(u8),      // change volume to this amount (processed beforehand I think)
    SkipToSeconds(u32), // intends to play current song from this time (bcs only active song can be target of this operation)
    SkipForwards,
    SkipBackwards,
    ToggleShuffle,            // will either shuffle or unshuffle the playlist
    GoToAlbum, // not implemented yet. will be used as change the surrounding playlist to the album the song is from
    ChangePlaylist(String), // change the current playlist to the one specified here
    PlayFromPlaylist(String), // used to play from a given playlist, at random.
}

#[derive(Debug, Clone)]
pub enum ProgramCommands {
    UpdateSender(Option<async_sender::UnboundedSender<PungeCommand>>),
    NewData(MusicData), // for sending back title, artist and album to GUI
    VolumeChange(u8),
    ShuffleToggle, // all of these play-type commands are
    PlayToggle,
    SkipForwards,
    SkipBackwards,
    StaticVolumeUp,
    StaticVolumeDown,
    SkipToSeconds(u32),
    MoveSlider(u32),
    ChangePage(Page),
    CheckBoxEvent(CheckBoxType, bool),
    Download(String),
    PlaylistResults(
        String,
        Result<rusty_ytdl::blocking::search::Playlist, AppError>,
    ),
    DownloadMedia(String, String, String), // link, path, mp3 or mp4
    DownloadMediaWorked(Result<String, AppError>), // to call when download media returns
    UpdateCombobox(ComboBoxType, String),  // little ugly lol. cause the type is a const
    SearchYouTube(String),
    SearchYouTubeResults(Vec<crate::types::YouTubeSearchResult>),
    Debug, // a message that has its associated action changed with the debug in question
    AddToDownloadFeedback(String, Result<YouTubeData, AppError>), // String = youtubelink, Result<string> = title - author
    InAppEvent(AppEvent),
    UpdateSearch(String), // for updating the string that is used in the regex search
    GoToSong,
    SongFound(Result<PungeMusicObject, AppError>), // when the song is found from GoToSong, this is called
    ChangeViewingPlaylist(String), // pass only the unqiueid i guess. problem was making self.viewing_playlist
    PlaySong(String),              // unqiueid
    MoveSongUp(String, usize),     // song_uuid, current position
    MoveSongDown(String, usize),   // song_uuid, current position
    AddToPlaylist(String, String), // uuid of playlist, uuid of song
    CreateBackup,
    UpdateWidgetText(TextType, String),
    SaveConfig,
    NewPlaylist,
    DeletePlaylist(String),
    UpdatePlaylist, // update playlist with the content from self.playlist_page
    OpenPlaylistEditPage(UserPlaylist), // i think this can be a &UserPlaylist ??
    ClearPlaylistPage,
    MovePlaylistUp(String),
    MovePlaylistDown(String),
    DuplicatePlaylist(String),
    PlayFromPlaylist(String), // right click menu on the playlist button. shuffles and plays randomly
    OpenSongEditPage(Option<String>),
    UpdateSong(crate::gui::widgets::row::RowData), // happens to be a convient type for this data
    QuickSwapTitleAuthor(String),                  // uniqueid
    PushScrubber(std::time::Duration),
    UpdateEditor(iced::widget::text_editor::Action), // updating the idle string editor in gui::settings
    // messages from the right-click menu on the table
    // playsong from above
    DeleteSong(String), // do this interface.. probably want some type of confirmation.. also the playlist is pulled from self.active_playlist or whatever its called
    SelectSong(usize, bool, String), // row, is_selected, uuid of song
    OnScroll(iced::widget::scrollable::Viewport),
    ValidatePlaylistData, // button to click in settings that re-checks the values of each playlists and sets them to their correct value
    InitiateDatabaseFix,
}

#[derive(Debug, Copy, Clone)]
pub enum TextType {
    // enum used in ProgramCommands::UpdateWidgetText(widget, text)
    // used to update the gui, and not need a bunch of different messages to get it done
    BackupText,         // settings
    Mp3Text,            // settings
    JpgText,            // settings
    StaticIncrement,    // settings
    StaticReduction,    // settings
    UserTitle,          // playlist
    UserDescription,    // playlist
    UserThumbnail,      // playlist
    Mp4DownloadInput,   // media
    Mp4PathInput,       // media
    TitleChange,        // song edit
    AuthorChange,       // song edit
    AlbumChange,        // song edit
    DownloadLinkInput,  // download page, input your own link
    YouTubeSearchInput, // download page, search for content on youtube
    MediaPath,          // settings page
}

#[derive(Clone, Debug)]
pub enum CheckBoxType {
    IncludeVideos,    // download page
    IncludePlaylists, // download page
}

#[derive(Clone, Debug)]
pub enum ComboBoxType {
    PlayKey,
    PlayModifier1,
    PlayModifier2,
    ForwardKey,
    ForwardModifer1,
    ForwardModifer2,
    BackwardKey,
    BackwardModifier1,
    BackwardModifier2,
    ShuffleKey,
    ShuffleModifier1,
    ShuffleModifier2,
    StaticUpKey,
    StaticUpModifier1,
    StaticUpModifier2,
    StaticDownKey,
    StaticDownModifier1,
    StaticDownModifier2,
    GoToAlbumKey,
    GoToAlbumModifier1,
    GoToAlbumModifer2,
    Mp3Or4, // media page
    ShuffleType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Main,
    Settings,
    Download,
    Media,
    Playlist,
    SongEdit, // cases where no page needs to be marked out (song_edit_page.rs)
}

#[derive(Debug, Clone)]
pub enum Context {
    Default, // when the app starts
    PlayPause,
    SkippedForward,
    SkippedBackwards,
    SkippedTo, // skipping to the seconds part
    Seeked,    // searching for a song!
    AutoPlay,
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    // will include in-app keybinds at some point...
    CloseRequested,
}

// this is only for the "PungeCommand"-like variants, PlayOrPause, ShuffleToggle..
// its so we can save these in cache
impl Serialize for ProgramCommands {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ProgramCommands::ShuffleToggle => {
                serializer.serialize_str("ProgramCommands::ShuffleToggle")
            }
            ProgramCommands::PlayToggle => serializer.serialize_str("ProgramCommands::PlayToggle"),
            ProgramCommands::SkipForwards => {
                serializer.serialize_str("ProgramCommands::SkipForwards")
            }
            ProgramCommands::SkipBackwards => {
                serializer.serialize_str("ProgramCommands::SkipBackwards")
            }
            ProgramCommands::StaticVolumeUp => {
                serializer.serialize_str("ProgramCommands::StaticVolumeUp")
            }
            ProgramCommands::StaticVolumeDown => {
                serializer.serialize_str("ProgramCommands::StaticVolumeDown")
            }
            _ => Err(ser::Error::custom(
                "Unsupported serialization of ProgramCommands member",
            )),
        }
    }
}

impl<'de> Deserialize<'de> for ProgramCommands {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String>::deserialize(deserializer)?;
        match s.as_str() {
            "ProgramCommands::PlayToggle" => Ok(ProgramCommands::PlayToggle),
            "ProgramCommands::SkipForwards" => Ok(ProgramCommands::SkipForwards),
            "ProgramCommands::SkipBackwards" => Ok(ProgramCommands::SkipBackwards),
            "ProgramCommands::StaticVolumeUp" => Ok(ProgramCommands::StaticVolumeUp),
            "ProgramCommands::StaticVolumeDown" => Ok(ProgramCommands::StaticVolumeDown),
            "ProgramCommands::ShuffleToggle" => Ok(ProgramCommands::ShuffleToggle),
            _ => Ok(ProgramCommands::Debug),
        }
    }
}
