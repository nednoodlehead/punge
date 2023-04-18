use std::fmt;
use chrono::{DateTime, Local, NaiveDate};


// object that will be returned, used to input into the database, this object is the
// object that will be returned from the whole process of deciding what is title, auth, album...
pub struct PungeMusicObject {
    pub title: String,
    pub author: String,
    pub album: String,
    pub features: String,
    pub savelocationmp3: String,
    pub savelocationjpg: String,
    pub datedownloaded: NaiveDate,
    pub lastlistenedto: NaiveDate,
    pub ischild: bool, // used in reconstruction of lost music that exists in DB
    pub uniqueid: String,
    pub plays: u16,
    pub weight: i16
}

pub struct Playlist {
    pub links: Vec<String>,
    pub title: String,
    pub author: String,
    pub length: u64
}

// this is the struct for making a playlist within the app. Not to be confused with playlist from youtube
pub struct UserPlaylist {
    pub title: String,
    pub description: String,
    pub thumbnail: String,  // path to thumbnail
    pub datecreated: NaiveDate,
    pub songcount: u16,
    pub totaltime: usize,  // updated each time a song is added or removed. in seconds
    pub isautogen: bool
}

impl fmt::Debug for Playlist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "title: {} \nauthor: {}\nlength: {}\nlinks: {:?}", &self.title, &self.author, &self.length, &self.links)
    }
}