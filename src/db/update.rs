use crate::types::{DatabaseErrors, PungeMusicObject};
use rusqlite::{params, Connection};

pub fn update_playlist(
    new_title: &str,
    description: &str,
    image: &str,
    uniqueid: &str,
) -> Result<(), DatabaseErrors> {
    // updates the title, description and image
    let conn = rusqlite::Connection::open("main.db")?;
    let statement: &str =
        "UPDATE metadata SET title = ?, description = ?, thumbnail = ? WHERE playlist_id = ?";
    conn.execute(statement, params![new_title, description, image, uniqueid])?;
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

pub fn delete_playlist(uniqueid: &str) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "DELETE FROM metadata WHERE playlist_id = ?",
        params![uniqueid],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

// for these we have to also check to see if it CAN go down one more
pub fn move_playlist_up_one(uniqueid: &str, count: u16) -> Result<(), DatabaseErrors> {
    let mut conn = Connection::open("main.db")?;
    // added a scope so the borrow is dropped :D
    if count == 0 {
        // cant go up any further...
        return Ok(());
    }
    let tx = conn.transaction()?;

    let new_count = count - 1;
    tx.execute(
        "UPDATE metadata SET userorder = ? WHERE userorder = ?",
        params![count, new_count],
    )?;
    tx.execute(
        "UPDATE metadata SET userorder = ? WHERE playlist_id = ?",
        params![new_count, uniqueid],
    )?;
    tx.commit()?;

    Ok(())
}

pub fn move_playlist_down_one(uniqueid: &str, count: u16) -> Result<(), DatabaseErrors> {
    let mut conn = Connection::open("main.db")?;

    let max: i64 = {
        let mut stmt = conn
            .prepare("SELECT COUNT(playlist_id) FROM metadata")
            .unwrap();
        stmt.query_row([], |row| row.get(0)).unwrap()
    };
    if count >= (max + 1) as u16 {
        return Ok(());
    }
    let tx = conn.transaction()?;
    let new_count = count + 1;

    tx.execute(
        "UPDATE metadata SET userorder = ? WHERE userorder = ?",
        params![count, new_count],
    )?;
    tx.execute(
        "UPDATE metadata SET userorder = ? WHERE playlist_id = ?",
        params![new_count, uniqueid],
    )?;
    tx.commit()?;

    Ok(())
}
