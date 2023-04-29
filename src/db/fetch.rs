use crate::playliststructs::{DatabaseErrors, PungeMusicObject};
use rusqlite::{params, Connection};
use std::fs::symlink_metadata;

pub fn get_all_from_playlist(playlist_uuid: &str) -> Result<Vec<PungeMusicObject>, DatabaseErrors> {
    // gets all songs from given table
    let conn = Connection::open("main.db")?;
    let mut stmt = conn.prepare("SELECT title, author, album, features,
    length, savelocationmp3, savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays,
    weight FROM main
    JOIN playlist_relations ON uniqueid = song_id
    WHERE playlist_id = ?")?;
    let punge_obj_iter = stmt.query_map([playlist_uuid], |row| {
        Ok(PungeMusicObject {
            title: row.get(0)?,
            author: row.get(1)?,
            album: row.get(2)?,
            features: row.get(3)?,
            length: row.get(4)?,
            savelocationmp3: row.get(5)?,
            savelocationjpg: row.get(6)?,
            datedownloaded: row.get(7)?,
            lastlistenedto: row.get(8)?,
            ischild: row.get(9)?,
            uniqueid: row.get(10)?,
            plays: row.get(11)?,
            weight: row.get(12)?,
        })
    })?;
    let mut ret_vec = Vec::new();
    for item in punge_obj_iter {
        ret_vec.push(item?)
    }
    drop(stmt);
    conn.close().map_err(|(_, err)| err)?;
    Ok(ret_vec)
}

pub fn get_all_main() -> Result<Vec<PungeMusicObject>, DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    let mut ret_vec: Vec<PungeMusicObject> = Vec::new();
    let mut stmt = conn.prepare("SELECT title, author, album, features,
    length, savelocationmp3, savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays,
    weight FROM main")?;
    let song_iter = stmt.query_map(params![], |row| {
        Ok(PungeMusicObject {
            title: row.get(0)?,
            author: row.get(1)?,
            album: row.get(2)?,
            features: row.get(3)?,
            length: row.get(4)?,
            savelocationmp3: row.get(5)?,
            savelocationjpg: row.get(6)?,
            datedownloaded: row.get(7)?,
            lastlistenedto: row.get(8)?,
            ischild: row.get(9)?,
            uniqueid: row.get(10)?,
            plays: row.get(11)?,
            weight: row.get(12)?,
        })
    })?;
    for obj in song_iter {
        ret_vec.push(obj?)
    }
    Ok(ret_vec)
}

// pub fn get_from_text_query(table: &str, query: &str) -> Vec<PungeMusicObject> {
//     // user input searches through all table entries, and if title, author, album, features.
//     // if it contains the user query, return that record
// }
//
// pub fn get_from_property_query(table: &str, field: &str, operator: &str) -> Vec<PungeMusicObject> {
//     // field and operator are from a preselected set of values
//     // operator: < > == !=
// }