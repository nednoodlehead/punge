// these are the messages sent around the program. This is divded up here because start.rs imports this
// as well as the actual music-playing portion of the app

#[derive(Debug, Clone)]
pub enum PungeCommand {
    Play,
    Stop,
    ChangeSong(usize), // play this song at this index in the list. also, do we need this as &str for thread safety?
    NewVolume(usize),  // change volume to this amount (processed beforehand I think)
    SkipToSeconds(usize),  // intends to play current song from this time (bcs only active song can be target of this operation)
    SkipForwards,
    SkipBackwards,
    StaticVolumeUp,  // used for binds to increase volume by x amount
    StaticVolumeDown,
    ToggleShuffle,  // will either shuffle or unshuffle the playlist
    GoToAlbum,  // not implemented yet. will be used as change the surrounding playlist to the album the song is from
    ChangePlaylist(String),  // change the current playlist to the one specified here
    None, // nothing burger
}