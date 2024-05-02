use crate::types::{DatabaseErrors, PungeMusicObject};
use rusqlite::{params, Connection};

pub fn _update_playlist(
    old_title: String,
    new_title: String,
    description: String,
    image: &str,
) -> Result<(), DatabaseErrors> {
    // updates the title, description and image
    let conn = rusqlite::Connection::open("main.db")?;
    let statement: &str =
        "UPDATE metadata SET title = ?, description = ?, image = ? WHERE title = ?";
    conn.execute(statement, params![new_title, description, image, old_title])?;
    let statement_2: String = format!("ALTER TABLE {} RENAME TO {}", old_title, new_title);
    conn.execute(statement_2.as_str(), params![])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn update_song(
    author: String,
    title: String,
    album: String,
    unique: String,
) -> Result<(), DatabaseErrors> {
    let conn: Connection = rusqlite::Connection::open("main.db")?;
    let statement: &str = "UPDATE main SET author = ?, title = ?, album = ? WHERE uniqueid = ?";
    conn.execute(statement, params![author, title, album, unique])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn _quick_swap_title_author(
    author: String,
    title: String,
    uniqueid: String,
) -> Result<(), DatabaseErrors> {
    let conn: Connection = rusqlite::Connection::open("main.db")?;
    let statement: &str = "UPDATE main SET author = ?, title = ? WHERE uniqueid = ?";
    conn.execute(statement, params![title, author, uniqueid])?;
    // conn.close() returns an err and connection. We drop the connection with .map_err()
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn update_empty_entries(obj: PungeMusicObject) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute("UPDATE main SET title = ?, author = ?, album = ?, features = ?, length = ?, savelocationmp3 = ?, savelocationjpg = ?, datedownloaded = ?, lastlistenedto = ?, ischild = ?, plays = ?, weight = ?, threshold = ? WHERE uniqueid = ?", params![
        obj.title, obj.author, obj.album, obj.features, obj.length, obj.savelocationmp3, obj.savelocationjpg, obj.datedownloaded, obj.lastlistenedto, obj.ischild, obj.plays, obj.weight, obj.threshold, obj.uniqueid
    ])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn delete_from_uuid(uniqueid: String) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute("DELETE FROM main WHERE uniqueid = ?", params![uniqueid])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn delete_from_playlist(uniqueid: String, playlistid: String) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "DELETE FROM playlist_relations WHERE playlist_id = ? AND song_id = ?",
        params![playlistid, uniqueid],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn update_auth_album(
    author: String,
    album: String,
    uniqueid: String,
) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "UPDATE main SET author = ?, album = ? WHERE uniqueid = ?",
        params![author, album, uniqueid],
    )?;

    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
pub fn update_title_auth(uniqueid: &str) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "UPDATE main SET author = title, title = author WHERE uniqueid = ?",
        params![uniqueid],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
