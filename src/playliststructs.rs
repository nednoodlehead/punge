use chrono::{DateTime, Local, NaiveDate};
use rusqlite::{types::FromSqlError, Connection, Error};
use std::fmt;
use thiserror::Error as terror;
use uuid::Uuid;

// object that will be returned, used to input into the database, this object is the
// object that will be returned from the whole process of deciding what is title, auth, album...
pub struct PungeMusicObject {
    pub title: String,
    pub author: String,
    pub album: String,
    pub features: String,
    pub length: String, // in seconds
    pub savelocationmp3: String,
    pub savelocationjpg: String,
    pub datedownloaded: NaiveDate,
    pub lastlistenedto: NaiveDate,
    pub ischild: bool, // used in reconstruction of lost music that exists in DB
    pub uniqueid: String,
    pub plays: u16,
    pub weight: i16,
}

pub struct Playlist {
    pub links: Vec<String>,
    pub title: String,
    pub author: String,
    pub length: u64,
}

// this is the struct for making a playlist within the app. Not to be confused with playlist from youtube
pub struct UserPlaylist {
    pub title: String,
    pub description: String,
    pub thumbnail: String, // path to thumbnail
    pub datecreated: NaiveDate,
    pub songcount: u16,
    pub totaltime: usize, // updated each time a song is added or removed. in seconds
    pub isautogen: bool,
    pub uniqueid: String,
}

impl UserPlaylist {
    pub fn new(title: String,
           description: String,
           thumbnail: String,
            isautogen: bool
            ) -> UserPlaylist {
        UserPlaylist {
            title, description, thumbnail, datecreated: Local::now().date_naive(), songcount: 0,
            totaltime: 0, isautogen, uniqueid: Uuid::new_v4().to_string()
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

#[derive(Debug, terror)]
pub enum DatabaseErrors {
    #[error("Rusqlite error: {0}")]
    RusqliteError(#[from] Error),
    #[error("FromSql error: {0}")]
    FromSqlError(#[from] FromSqlError),
    #[error("File Already Exists")]
    FileExistsError,  // used when a song already downloaded
    #[error("UniqueID Already Present in DB")]
    DatabaseEntryExistsError  // used when the unique id is already present in the database
}
