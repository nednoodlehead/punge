// these are the messages sent around the program. This is divded up here because start.rs imports this
// as well as the actual music-playing portion of the app

use crate::types::{AppError, PungeMusicObject};
use crate::types::{MusicData, YouTubeData};
use iced::widget::scrollable;
use tokio::sync::mpsc as async_sender;
#[derive(Debug, Clone)]
pub enum PungeCommand {
    PlayOrPause,
    ChangeSong(String), // play this song's uuid, loop will find the index and swap to it
    NewVolume(u8),      // change volume to this amount (processed beforehand I think)
    SkipToSeconds(u32), // intends to play current song from this time (bcs only active song can be target of this operation)
    SkipForwards,
    SkipBackwards,
    ToggleShuffle,          // will either shuffle or unshuffle the playlist
    GoToAlbum, // not implemented yet. will be used as change the surrounding playlist to the album the song is from
    ChangePlaylist(String), // change the current playlist to the one specified here
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
    UpdateDownloadEntry(String),
    Download(String),
    DownloadMedia(String, String, String), // link, path, mp3 or mp4
    DownloadMediaWorked(Result<String, AppError>), // to call when download media returns
    UpdateMp3Or4Combobox(String),
    SearchYouTube(String),
    SearchYouTubeResults(Vec<crate::types::YouTubeSearchResult>),
    Debug, // a message that has its associated action changed with the debug in question
    AddToDownloadFeedback(String, Result<YouTubeData, AppError>), // String = youtubelink, Result<string> = title - author
    InAppEvent(AppEvent),
    UpdateSearch(String), // for updating the string that is used in the regex search
    GoToSong,             //
    SongFound(Result<PungeMusicObject, AppError>), // when the song is found from GoToSong, this is called
    ChangeViewingPlaylist(String), // pass only the unqiueid i guess. problem was making self.viewing_playlist
    PlaySong(String),              // unqiueid
    SelectSong(String, String),    // uniqueid and title, used to do stuff to the current song
    DeleteSong(String),
    SyncHeader(scrollable::AbsoluteOffset), // not used, could revamp table tbh..
    PlaylistSelected(String), // playlist uuid, would love to also pass in title, but cannot due to pick_list restrictions :(
    AddToPlaylist(Option<String>, Option<String>), // add song uniqueid and playlist uniqueid
    ToggleList,
    CreateBackup,
    UpdateWidgetText(TextType, String),
    SaveConfig,
    NewPlaylist,
    OpenSongEditPage(String),           // uuid
    UpdateSong(crate::gui::table::Row), // happens to be a convient type for this data
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
    Seeked,
    AutoPlay,
}

#[derive(Clone, Debug)]
pub enum AppEvent {
    // will include in-app keybinds at some point...
    CloseRequested,
}
