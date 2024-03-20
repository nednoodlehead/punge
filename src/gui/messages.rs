// these are the messages sent around the program. This is divded up here because start.rs imports this
// as well as the actual music-playing portion of the app

use crate::types::{AppError, UserPlaylist};
use crate::types::{MusicData, YouTubeData};
use iced::widget::scrollable;
use tokio::sync::mpsc as async_sender;
#[derive(Debug, Clone)]
pub enum PungeCommand {
    PlayOrPause,
    ChangeSong(String), // play this song's uuid, loop will find the index and swap to it
    NewVolume(u8),      // change volume to this amount (processed beforehand I think)
    SkipToSeconds(usize), // intends to play current song from this time (bcs only active song can be target of this operation)
    SkipForwards,
    SkipBackwards,
    StaticVolumeUp, // used for binds to increase volume by x amount
    StaticVolumeDown,
    ToggleShuffle,          // will either shuffle or unshuffle the playlist
    GoToAlbum, // not implemented yet. will be used as change the surrounding playlist to the album the song is from
    ChangePlaylist(String), // change the current playlist to the one specified here
    NewStatic(f32, f32),
}

#[derive(Debug, Clone)]
pub enum ProgramCommands {
    Send(PungeCommand),
    UpdateSender(Option<async_sender::UnboundedSender<PungeCommand>>),
    NewData(MusicData), // for sending back title, artist and album to GUI
    VolumeChange(u8),
    ShuffleToggle,
    PlayToggle,
    SkipForwards,
    SkipBackwards,
    ChangePage(Page),
    UpdateDownloadEntry(String),
    Download(String),
    Debug, // a message that has its associated action changed with the debug in question
    AddToDownloadFeedback(Option<Result<YouTubeData, AppError>>), // only called from the subscription,
    InAppEvent(AppEvent),
    UpdateSearch(String), // for updating the string that is used in the regex search
    GoToSong, // uses the regex search to take user input and skip to nearest search for user. input derives from self.search
    ChangeViewingPlaylist(UserPlaylist), // pass whole playlist in.
    ChangeActivePlaylist(UserPlaylist),
    PlaySong(String),           // unqiueid
    SelectSong(String, String), // uniqueid and title, used to do stuff to the current song
    SyncHeader(scrollable::AbsoluteOffset),
    PlaylistSelected(String), // playlist uuid, would love to also pass in title, but cannot due to pick_list restrictions :(
    AddToPlaylist(Option<String>, Option<String>), // add song uniqueid and playlist uniqueid
    ToggleList,
    CreateBackup,
    UpdateWidgetText(TextType, String),
    SaveConfig,
    NewPlaylist, // title, description, path_to_thumbnail
}

#[derive(Debug, Copy, Clone)]
pub enum TextType {
    // enum used in ProgramCommands::UpdateWidgetText(widget, text)
    // used to update the gui, and not need a bunch of different messages to get it done
    BackupText,      // settings
    Mp3Text,         // settings
    JpgText,         // settings
    StaticIncrement, // settings
    StaticReduction, // settings
    UserTitle,       // playlist
    UserDescription, // playlist
    UserThumbnail,   // playlist
}

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Main,
    Settings,
    Download,
    Media, // TODO make the media page :)
    Playlist,
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
