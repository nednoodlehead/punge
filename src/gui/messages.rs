// these are the messages sent around the program. This is divded up here because start.rs imports this
// as well as the actual music-playing portion of the app

use crate::gui::start::MusicData;
use crate::playliststructs::{AppError, UserPlaylist};
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
    None,      // nothing burger
}

#[derive(Debug, Clone)]
pub enum ProgramCommands {
    Test,
    Send(PungeCommand),
    UpdateSender(Option<async_sender::UnboundedSender<PungeCommand>>),
    NewData(MusicData), // for sending back title, artist and album to GUI
    VolumeChange(u8),
    DownloadLink(String),
    ChangePage(Page),
    UpdateDownloadEntry(String),
    Download(String),
    Debug, // a message that has its associated action changed with the debug in question
    AddToDownloadFeedback(Option<Vec<Result<(String, String), AppError>>>), // only called from the subscription,
    InAppEvent(AppEvent),
    UpdateSearch(String), // for updating the string that is used in the regex search
    GoToSong, // uses the regex search to take user input and skip to nearest search for user. input derives from self.search
    ChangeViewingPlaylist(UserPlaylist), // pass whole playlist in.
    ChangeActivePlaylist(UserPlaylist),
}

#[derive(Debug, Clone, Copy)]
pub enum Page {
    Main,
    Settings,
    Download,
    Media,
}

#[derive(Debug, Clone)]
// these are all the actions the user can perform that can change the weight / plays of a song
pub enum DatabaseMessages {
    Played(String), // uuid passed in
    Skipped(String),
    Seeked(String),
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
