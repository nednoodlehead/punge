use crate::playliststructs::{DatabaseErrors, PungeMusicObject};
use rusqlite::{params, Connection};
use std::fs::symlink_metadata;

pub fn get_all_from_playlist(playlist_uuid: &str) -> Result<Vec<PungeMusicObject>, DatabaseErrors> {
    // gets all songs from given table
    let conn = Connection::open("main.db")?;
    let mut stmt = conn.prepare("SELECT title, author, album, features,
    length, savelocationmp3, savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays,
    weight, threshold FROM main
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
            threshold: row.get(13)?,
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
    // erm, forgot to close the db connection :nerd:
    let conn = Connection::open("main.db")?;
    let mut ret_vec: Vec<PungeMusicObject> = Vec::new();
    let mut stmt = conn.prepare("SELECT title, author, album, features,
    length, savelocationmp3, savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays,
    weight, threshold FROM main")?;
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
            threshold: row.get(13)?,
        })
    })?;
    for obj in song_iter {
        ret_vec.push(obj?)
    }
    Ok(ret_vec)
}

pub fn exists_in_db(uniqueid: String) -> bool {
    let conn = Connection::open("main.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT title FROM main WHERE uniqueid = ?")
        .unwrap();
    let mut rows = stmt.query(&[&uniqueid]).unwrap();
    let val = rows.next().unwrap().is_some();
    drop(rows); // drop to release borrown on stmt
    drop(stmt); // explicitly drop stmt to release borrow on conn
    conn.close().unwrap();
    val
}

pub fn get_uuid_from_name(playlist_name: String) -> String {
    let conn = Connection::open("main.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT playlist_id from metadata WHERE title = ?")
        .unwrap();
    let mut result: String = stmt.query_row(&[&playlist_name], |row| row.get(0)).unwrap();
    drop(stmt);
    conn.close().unwrap();
    result
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

use crate::playliststructs::UserPlaylist;

pub fn get_all_playlists() -> Result<Vec<UserPlaylist>, DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    let mut stmt = conn.prepare("SELECT title, description, thumbnail, datecreated, songcount, totaltime, isautogen, playlist_id
        FROM metadata")?;
    let playlist_obj_iter = stmt.query_map([], |row| {
        Ok(UserPlaylist {
            title: row.get(0)?,
            description: row.get(1)?,
            thumbnail: row.get(2)?,
            datecreated: row.get(3)?,
            songcount: row.get(4)?,
            totaltime: row.get(5)?,
            isautogen: row.get(6)?,
            uniqueid: row.get(7)?,
        })
    })?;
    let mut ret_vec = Vec::new();
    for item in playlist_obj_iter {
        ret_vec.push(item?)
    }
    drop(stmt);
    conn.close().map_err(|(_, err)| err)?;
    Ok(ret_vec)
}
