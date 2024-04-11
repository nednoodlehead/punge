use chrono::{Local, NaiveDate};
use rusqlite::{types::FromSqlError, Error};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error as terror;
use uuid::Uuid;
// object that will be returned, used to input into the database, this object is the
// object that will be returned from the whole process of deciding what is title, auth, album...

#[derive(Clone)]
pub struct PungeMusicObject {
    pub title: String,
    pub author: String,
    pub album: String,
    pub features: String,
    pub length: u32, // in seconds
    pub savelocationmp3: String,
    pub savelocationjpg: String,
    pub datedownloaded: NaiveDate,
    pub lastlistenedto: NaiveDate,
    pub ischild: bool, // used in reconstruction of lost music that exists in DB
    pub uniqueid: String,
    pub plays: u16,
    pub weight: i16,
    pub threshold: u16,
}

pub struct Playlist {
    pub links: Vec<String>,
    pub title: String,
    pub author: String,
    pub length: u64,
}

// this is the struct for making a playlist within the app. Not to be confused with playlist from youtube
#[derive(Clone)]
pub struct UserPlaylist {
    pub title: String,
    pub description: String,
    pub thumbnail: String, // path to thumbnail
    pub datecreated: NaiveDate,
    pub songcount: u16,
    pub totaltime: String, // updated each time a song is added or removed. in seconds
    pub isautogen: bool,
    pub uniqueid: String,
}

impl std::fmt::Display for UserPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl std::fmt::Debug for UserPlaylist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "title:{} | id: {}", self.title, self.uniqueid)
    }
}

impl PartialEq for UserPlaylist {
    fn eq(&self, other: &Self) -> bool {
        self.uniqueid == other.uniqueid
    }
}

impl UserPlaylist {
    pub fn new(
        title: String,
        description: String,
        thumbnail: String,
        isautogen: bool,
    ) -> UserPlaylist {
        UserPlaylist {
            title,
            description,
            thumbnail,
            datecreated: Local::now().date_naive(),
            songcount: 0,
            totaltime: "0".to_string(),
            isautogen,
            uniqueid: Uuid::new_v4().to_string(),
        }
    }
}
impl fmt::Debug for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {} \nauthor: {}\nlength: {}\nlinks: {:?}",
            &self.title, &self.author, &self.length, &self.links
        )
    }
}

impl fmt::Debug for PungeMusicObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "title: {} author: {} unique: {}",
            &self.title, self.author, self.uniqueid
        )
    }
}

// wrap the two errors that can arise from database problems into our own custom enum

#[derive(Debug, terror, Clone)]
pub enum DatabaseErrors {
    #[error("File Already Exists")]
    FileExistsError, // used when a song already downloaded
    #[error("UniqueID Already Present in DB")]
    DatabaseEntryExistsError, // used when the unique id is already present in the database
    #[error("Error inserting")]
    FromSqlError(String),
}

#[derive(Debug, Clone)]
pub enum AppError {
    DatabaseError(DatabaseErrors),
    YoutubeError(String), // url, what went wrong
    FfmpegError(String),
    FileError(String),
    InvalidUrlError(String),
    YouTubeError(String),
    SearchError(String),
}

impl From<DatabaseErrors> for AppError {
    fn from(error: DatabaseErrors) -> Self {
        AppError::DatabaseError(error)
    }
}

use rusty_ytdl::VideoError;
impl From<VideoError> for AppError {
    fn from(e: VideoError) -> Self {
        AppError::YoutubeError(e.to_string())
    }
}

impl From<FromSqlError> for AppError {
    fn from(e: FromSqlError) -> Self {
        AppError::DatabaseError(DatabaseErrors::FromSqlError(e.to_string()))
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: Error) -> Self {
        AppError::DatabaseError(DatabaseErrors::FromSqlError(e.to_string()))
    }
}

impl From<rusqlite::Error> for DatabaseErrors {
    fn from(e: Error) -> Self {
        DatabaseErrors::FromSqlError(e.to_string())
    }
}
use crate::gui::messages::Context;
#[derive(Clone, Debug)]
pub struct MusicData {
    // passed from music subscription -> main thread
    pub title: String, // used to updated active songs and whatnot
    pub author: String,
    pub album: String,
    pub song_id: String,
    pub previous_id: Option<String>, // used only inside of skip_forward database subscription, None otherwise.
    pub volume: f32,
    pub is_playing: bool,
    pub shuffle: bool,
    pub playlist: String,
    pub threshold: u16,
    pub context: Context, // the context of the message being sent
    pub length: u32,      // seconds, length of song
}

impl MusicData {
    pub fn default() -> Self {
        MusicData {
            title: "".to_string(),
            author: "".to_string(),
            album: "".to_string(),
            song_id: "".to_string(),
            previous_id: None,
            volume: 0.0,
            is_playing: false,
            shuffle: false,
            playlist: "main".to_string(),
            threshold: 0,
            context: Context::Default,
            length: 0,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    // no light mode will be made . final decision
    pub backup_path: String,
    pub mp3_path: String,
    pub jpg_path: String,
    pub static_increment: usize,
    pub static_reduction: usize,
    pub media_path: String, // default location for media
}

// used in src/yt to move data around in an easier / simpler format
#[derive(Debug, Clone)]
pub struct YouTubeData {
    pub title: String,
    pub author: String,
    pub album: String,
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct YouTubeSearchResult {
    pub title: String,
    pub author: String,
    pub views: u64,
    pub duration: Option<String>, // duration of video "10:10", "2:45"
    pub videos: Option<String>,   // format: {} Videos, videos.len()
    pub thumbnail: String,        // path to the thumbnail
    pub link: String,
}
