
use rustqlite::Connection;
use crate::playliststructs::PungeMusicObject;


pub fn get_all(table: &str) -> Vec<PungeMusicObject> {
    // gets all songs from given table
}

pub fn get_from_text_query(table: &str, query: &str) -> Vec<PungeMusicObject> {
    // user input searches through all table entries, and if title, author, album, features.
    // if it contains the user query, return that record
}

pub fn get_from_property_query(table: &str, field: &str, operator: &str) -> Vec<PungeMusicObject> {
    // field and operator are from a preselected set of values
    // operator: < > == !=
}