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

pub fn quick_swap_title_author(uniqueid: &str) -> Result<(), DatabaseErrors> {
    let conn: Connection = rusqlite::Connection::open("main.db")?;
    let statement: &str = "UPDATE main SET author = title, title = author WHERE uniqueid = ?";
    conn.execute(statement, params![uniqueid])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn delete_from_uuid(uniqueid: String) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute("DELETE FROM main WHERE uniqueid = ?", params![uniqueid])?;
    conn.execute(
        "UPDATE metadata SET songcount = songcount - 1 WHERE playlist_id = main",
        params![],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn delete_from_playlist(uniqueid: String, playlistid: String) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "DELETE FROM playlist_relations WHERE playlist_id = ? AND song_id = ?",
        params![&playlistid, &uniqueid],
    )?;
    conn.execute(
        "UPDATE metadata SET songcount = songcount -1 WHERE playlist_id = ?",
        params![&playlistid],
    )?;
    conn.execute(
        "UPDATE metadata SET totaltime = totaltime - (SELECT length FROM main WHERE uniqueid = ?) WHERE playlist_id = ?",
        params![uniqueid, playlistid],
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
        "UPDATE metadata SET order_of_playlist = ? WHERE order_of_playlist = ?",
        params![count, new_count],
    )?;
    tx.execute(
        "UPDATE metadata SET order_of_playlist = ? WHERE playlist_id = ?",
        params![new_count, uniqueid],
    )?;
    tx.commit()?;

    Ok(())
}

pub fn move_song_up_one(
    // ok so it sort of just occured to me that we could skip the whole 'uuid' part and just have a single number
    // then make the var (one above or below depending on up or down song) then atomic swap them...
    song_uuid: String,
    position: usize,
    playlist_uuid: String,
) -> Result<(), DatabaseErrors> {
    // we must differenciate between a change on 'main' and playlist, since the sql is different
    let mut conn = Connection::open("main.db")?;
    if playlist_uuid != "main" {
        // set the new number's number to +1
        conn.execute("UPDATE playlist_relations SET user_playlist_order = user_playlist_order - 1 WHERE user_playlist_order = ? AND playlist_id = ?", params![position, &playlist_uuid])?;
        // update the one we are moving up to the new number
        conn.execute(
            "UPDATE playlist_relations SET user_playlist_order =  ",
            params![position, &song_uuid, &playlist_uuid],
        )?;
    } else {
        let trans = conn.transaction()?;
        let one_below = position - 1;
        trans.execute(
            "UPDATE main SET user_order = ? WHERE user_order = ?",
            params![position, one_below],
        )?;
        trans.execute(
            "UPDATE main SET user_order = ? WHERE uniqueid = ?",
            params![one_below, song_uuid],
        )?;
        trans.commit()?;
    }
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
// like i guess you could make these ^ & v one function? maybe something to refactor *one* day :)
pub fn move_song_down_one(
    song_uuid: String,
    position: usize,
    playlist_uuid: String,
) -> Result<(), DatabaseErrors> {
    // we must differenciate between a change on 'main' and playlist, since the sql is different
    let mut conn = Connection::open("main.db")?;
    if playlist_uuid != "main" {
        // set the new number's number to +1
        conn.execute("UPDATE playlist_relations SET user_playlist_order = user_playlist_order - 1 WHERE user_playlist_order = ? AND playlist_id = ?", params![position, &playlist_uuid])?;
        // update the one we are moving up to the new number
        conn.execute(
            "UPDATE playlist_relations SET user_playlist_order =  ",
            params![position, &song_uuid, &playlist_uuid],
        )?;
    } else {
        // the one we are affecting but didnt select
        let trans = conn.transaction()?; // mowt says trans rights
                                         // so the one we care about will go up in value, the other will go "down" (referring to visual, not numerical)
        let one_above = position + 1;
        trans.execute(
            "UPDATE main SET user_order = ? WHERE user_order = ?",
            params![position, one_above],
        )?;
        trans.execute(
            "UPDATE main SET user_order = ? WHERE uniqueid = ?",
            params![one_above, song_uuid],
        )?;
        trans.commit()?;
    }
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
