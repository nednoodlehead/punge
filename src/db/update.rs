use crate::types::{DatabaseErrors, UserPlaylist};
use log::{debug, info};
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

pub fn delete_from_uuid(uniqueid: &str) -> Result<(), DatabaseErrors> {
    let mut conn = Connection::open("main.db")?;
    // just realized that we need to decrement every single entry in main (> it's count) so the count stays accurate
    // also if anything fails, it will f up the db a bit
    let trans = conn.transaction()?;
    trans.execute("UPDATE main SET user_order = user_order -1 WHERE user_order > (SELECT user_order from main WHERE uniqueid = ?)", params![uniqueid])?;
    trans.execute("DELETE FROM main WHERE uniqueid = ?", params![uniqueid])?;
    trans.execute(
        "UPDATE metadata SET songcount = songcount - 1 WHERE playlist_id = 'main'",
        params![],
    )?;
    trans.commit()?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn delete_from_playlist(uniqueid: &str, playlistid: &str) -> Result<(), DatabaseErrors> {
    let mut conn = Connection::open("main.db")?;
    let trans = conn.transaction()?;
    trans.execute(
        "DELETE FROM playlist_relations WHERE playlist_id = ? AND song_id = ?",
        params![playlistid, uniqueid],
    )?;
    trans.execute(
        "UPDATE metadata SET songcount = songcount -1 WHERE playlist_id = ?",
        params![playlistid],
    )?;
    trans.execute(
        "UPDATE metadata SET totaltime = totaltime - (SELECT length FROM main WHERE uniqueid = ?) WHERE playlist_id = ?",
        params![uniqueid, playlistid],
    )?;
    trans.commit()?;
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

pub fn delete_playlist(uniqueid: &str) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "DELETE FROM metadata WHERE playlist_id = ?",
        params![uniqueid],
    )?;
    conn.execute(
        "DELETE FROM playlist_relations WHERE playlist_id = ?",
        params![uniqueid],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

// for these we have to also check to see if it CAN go down one more
pub fn move_playlist_up_one(uniqueid: &str) -> Result<(), DatabaseErrors> {
    let mut conn = Connection::open("main.db")?;
    // added a scope so the borrow is dropped :D
    let mut prep = conn
        .prepare("SELECT order_of_playlist FROM metadata WHERE playlist_id = ?")
        .unwrap();
    let count: usize = prep.query_row([uniqueid], |row| row.get(0)).unwrap();
    if count == 0 {
        // cant go up any further...
        return Ok(());
    }
    drop(prep);
    let tx = conn.transaction()?;

    let new_count = count - 1;
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

pub fn move_playlist_down_one(uniqueid: &str) -> Result<(), DatabaseErrors> {
    let count = 0;
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
    let mut prep = conn
        .prepare("SELECT order_of_playlist FROM metadata WHERE playlist_id = ?")
        .unwrap();
    let count: usize = prep.query_row([uniqueid], |row| row.get(0)).unwrap();
    drop(prep);
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
        // set the new number's number to -1
        let trans = conn.transaction()?;
        let one_below = position - 1;
        // sets the selected one correctly. moves the target's number up (0 -> 1)
        debug!("moving {} -> {}", position, one_below);
        trans.execute("UPDATE playlist_relations SET user_playlist_order = ? WHERE playlist_id = ? AND user_playlist_order = ?", params![position, &playlist_uuid, one_below])?;
        // should set the other
        debug!("and now setting id: {} -> {}", &song_uuid, one_below);
        trans.execute("UPDATE playlist_relations SET user_playlist_order = ? WHERE playlist_id = ? AND song_id = ?", params![one_below, playlist_uuid, song_uuid])?;
        trans.commit()?;
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
        let one_above = position + 1;
        let trans = conn.transaction()?;
        debug!("moving on playlist: {} -> {}", position, one_above);
        trans.execute("UPDATE playlist_relations SET user_playlist_order = ? WHERE playlist_id = ? AND user_playlist_order = ?", params![position, &playlist_uuid, one_above])?;
        // should set the other
        debug!("and now setting id: {} -> {}", &song_uuid, one_above);
        trans.execute("UPDATE playlist_relations SET user_playlist_order = ? WHERE playlist_id = ? AND song_id = ?", params![one_above, playlist_uuid, song_uuid])?;
        trans.commit()?;
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

pub fn duplicate_playlist(playlistid: &str) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    info!("duplicating: {}", playlistid);
    let mut stmt = conn.prepare(
        "SELECT title, description, thumbnail, datecreated, songcount, totaltime, isautogen, order_of_playlist FROM metadata WHERE playlist_id = ?",
    )?;
    let new_uuid = uuid::Uuid::new_v4();
    let mut obj = stmt
        .query_row([playlistid], |row| {
            Ok(UserPlaylist {
                title: row.get(0)?,
                description: row.get(1)?,
                thumbnail: row.get(2)?,
                datecreated: row.get(3)?,
                songcount: row.get(4)?,
                totaltime: row.get(5)?,
                isautogen: row.get(6)?,
                userorder: row.get(7)?,
                scrollable_offset: iced::widget::scrollable::AbsoluteOffset::default(), // doesnt matter..
                uniqueid: new_uuid.to_string(),
            })
        })
        .unwrap();
    drop(stmt);
    obj.title = format!("{} Dupe", obj.title);
    obj.userorder = crate::db::fetch::get_num_of_playlists() + 1;
    crate::db::insert::create_playlist(obj).unwrap();
    // copy all of the entries in playlist_relations that include the original playlistid, and copy them to the new one
    let mut stmt2 = conn.prepare(
        "SELECT song_id FROM playlist_relations WHERE playlist_id = ? ORDER BY user_playlist_order",
    )?;
    // you're probably thinking: ned, you moron. why use two loops for this!? just do it in one
    // and i tell you: SqliteFailure(Error Databasebusy)
    let mut tmp = vec![];
    let iter_entries = stmt2.query_map([playlistid], |row| Ok(row.get::<_, String>(0)?))?;
    for item in iter_entries {
        tmp.push(item.unwrap())
    }
    for (count, string) in tmp.iter().enumerate() {
        crate::db::insert::add_to_playlist_silent(&new_uuid.to_string(), &string, count)
    }
    Ok(())
}
// must be called at some point to update the offset. probably after swapping playlists? Might be sucky performance? Or alternatively on exit.
pub fn update_offset(playlistid: &str, offset: f32) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    info!("writing offset to uuid: {}", playlistid);
    conn.execute(
        "UPDATE metadata SET table_offset = ? WHERE playlist_id =?",
        params![offset, playlistid],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

struct Validate {
    title: String,
    songcount: usize,
    totaltime: String,
    playlist_id: String,
}

pub fn validate_playlist_data() -> Result<(), DatabaseErrors> {
    // first we fetch the values, and log them. afterwards, we log the new values, so the user can check and see if something changed
    let conn = Connection::open("main.db")?;
    let mut stmt = conn.prepare("SELECT title, songcount, totaltime, playlist_id FROM metadata")?;
    let iter_ent = stmt.query_map(params![], |row| {
        Ok(Validate {
            title: row.get(0)?,
            songcount: row.get(1)?,
            totaltime: row.get(2)?,
            playlist_id: row.get(3)?,
        })
    })?;
    for entry in iter_ent {
        let it = entry.unwrap();
        info!(
            "checking before the updates {}: {} {}",
            it.title, it.songcount, it.totaltime
        );
        if it.title != "Main" {
            let mut stmt2 = conn.prepare("SELECT COUNT(title) FROM metadata WHERE title = ?")?;
            let count: i64 = stmt2.query_row([&it.title], |row| row.get(0))?;
            conn.execute(
                "UPDATE metadata SET songcount =? WHERE playlist_id = ?",
                params![count, &it.playlist_id],
            )?;
            info!(
                "count of {} was {} and is now {}",
                &it.title, it.songcount, count
            );
            let mut stmt3 =
                conn.prepare("SELECT length FROM main JOIN playlist_relations ON uniqueid = song_id WHERE playlist_id = ?")?;
            let song_total =
                stmt3.query_map(params![it.playlist_id], |row| Ok(row.get::<_, u16>(0)?))?;
            let mut complete_song_total = 0;
            let mut sub_count = 0;
            for song in song_total {
                complete_song_total += song.unwrap();
                sub_count += 1;
            }
            info!(
                "total song length of {} is {} ({} total len)",
                it.title, complete_song_total, sub_count
            );
            conn.execute(
                "UPDATE metadata SET (songcount, totaltime) = (?, ?) WHERE playlist_id = ?",
                params![sub_count, complete_song_total, it.playlist_id],
            )?;
        } else {
            // this is just for main
            let mut main_stmt = conn.prepare("SELECT length FROM main")?;
            let items = main_stmt.query_map(params![], |row| Ok(row.get::<usize, i64>(0)?))?;
            let mut total_time = 0;
            let mut count = 0;
            for x in items.into_iter() {
                total_time += x.unwrap();
                count += 1;
            }
            info!(
                "main appears to have len of: {} and total of {}",
                count, total_time
            );
            conn.execute(
                "UPDATE metadata SET (songcount, totaltime) = (?, ?) WHERE playlist_id = \"main\"",
                params![count, total_time],
            )?;
        };
    }
    Ok(())
}
