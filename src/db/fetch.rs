use crate::types::{DatabaseErrors, PungeMusicObject};

use rusqlite::{params, Connection};

pub fn get_all_from_playlist(playlist_uuid: &str) -> Result<Vec<PungeMusicObject>, DatabaseErrors> {
    // gets all songs from given table
    let conn = Connection::open("main.db")?;
    let mut stmt = conn.prepare("SELECT title, author, album, features,
    length, savelocationmp3, savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays,
    weight, threshold, user_order FROM main
    JOIN playlist_relations ON uniqueid = song_id
    WHERE playlist_id = ?
    ORDER BY user_playlist_order")?;
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
            order: row.get(14)?,
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
    weight, threshold, user_order FROM main ORDER BY user_order")?;
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
            order: row.get(14)?,
        })
    })?;
    for obj in song_iter {
        ret_vec.push(obj?)
    }
    Ok(ret_vec)
}

pub fn _exists_in_db(uniqueid: String) -> bool {
    let conn = Connection::open("main.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT title FROM main WHERE uniqueid = ?")
        .unwrap();
    let mut rows = stmt.query([&uniqueid]).unwrap();
    let val = rows.next().unwrap().is_some();
    drop(rows); // drop to release borrown on stmt
    drop(stmt); // explicitly drop stmt to release borrow on conn
    conn.close().unwrap();
    val
}

pub fn get_num_of_playlists() -> u16 {
    let conn = Connection::open("main.db").unwrap();
    let mut stmt = conn
        .prepare("SELECT COUNT(playlist_id) FROM metadata")
        .unwrap();
    let count: i64 = stmt.query_row([], |row| row.get(0)).unwrap();
    count as u16
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

use crate::types::UserPlaylist;

pub fn get_all_playlists() -> Result<Vec<UserPlaylist>, DatabaseErrors> {
    // we assume that the user has a 'main' playlist
    let conn = Connection::open("main.db")?;
    let mut stmt = conn.prepare("SELECT title, description, thumbnail, datecreated, songcount, totaltime, isautogen, order_of_playlist, playlist_id
        FROM metadata ORDER BY order_of_playlist")?;
    let playlist_obj_iter = stmt.query_map([], |row| {
        Ok(UserPlaylist {
            title: row.get(0)?,
            description: row.get(1)?,
            thumbnail: row.get(2)?,
            datecreated: row.get(3)?,
            songcount: row.get(4)?,
            totaltime: row.get(5)?,
            isautogen: row.get(6)?,
            userorder: row.get(7)?,
            uniqueid: row.get(8)?,
        })
    })?;
    let mut ret_vec = Vec::new();
    for item in playlist_obj_iter {
        println!("item: {:?}", &item);
        ret_vec.push(item?)
    }
    println!("order b4: {:?}", &ret_vec);
    ret_vec.sort_unstable_by_key(|item| item.userorder);
    println!("order after: {:?}", &ret_vec);
    drop(stmt);
    conn.close().map_err(|(_, err)| err)?;
    Ok(ret_vec)
}

pub fn get_obj_from_uuid(uniqueid: &str) -> Result<PungeMusicObject, DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    let mut stmt = conn.prepare("SELECT * from main where uniqueid =?")?;
    let playlist_obj_iter = stmt.query_row([uniqueid], |row| {
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
            order: row.get(14)?,
        })
    })?;
    Ok(playlist_obj_iter)
}
