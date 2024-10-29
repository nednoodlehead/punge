use crate::types::{DatabaseErrors, PungeMusicObject, UserPlaylist};
use log::info;
use rusqlite::{params, Connection};

pub fn add_to_main(music_obj: PungeMusicObject) -> Result<String, DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    info!("Adding into main!");
    conn.execute("INSERT INTO \"main\" (title, author, album, features, length, savelocationmp3,\
                    savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays, weight, threshold, user_order)\
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                 params![music_obj.title, music_obj.author, music_obj.album, music_obj.features, music_obj.length, music_obj.savelocationmp3,
                 music_obj.savelocationjpg, music_obj.datedownloaded, music_obj.lastlistenedto, music_obj.ischild, music_obj.uniqueid,
                 music_obj.plays, music_obj.weight, music_obj.threshold, music_obj.order])?;
    // untested... should work..?
    conn.execute(
        "UPDATE metadata SET songcount = songcount + 1, totaltime = totaltime + ? WHERE playlist_id = \"main\"",
        params![music_obj.length],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(format!("{} - {}", &music_obj.title, &music_obj.author))
}

pub fn create_playlist(new_playlist: UserPlaylist) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    // check if that table already exists
    info!("inserting, playlist does not exist");
    conn.execute(
        "INSERT INTO metadata (title, description, thumbnail, datecreated,
        songcount, totaltime, isautogen, table_offset, order_of_playlist, playlist_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            new_playlist.title,
            new_playlist.description,
            new_playlist.thumbnail,
            new_playlist.datecreated,
            new_playlist.songcount,
            new_playlist.totaltime,
            new_playlist.isautogen,
            0.0,
            new_playlist.userorder,
            new_playlist.uniqueid
        ],
    )?;

    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

// although it would seem intelligent to include the length of the song, there is nowhere where that data can really live, and make sense
// like when we select from the table, should length be included? sounds sort of stupid. so we just query it here...
/// order num is the current length of the playlist. so if there is 3 songs, order_num should be three
pub fn _add_to_playlist_bulk(
    playlist_uuid: &str,
    uniqueid: Vec<String>,
    order_num: usize,
) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    for (count, special_id) in uniqueid.iter().enumerate() {
        conn.execute(
            "INSERT INTO playlist_relations (playlist_id, song_id, user_playlist_order) VALUES (?1, ?2, ?3)",
            params![&playlist_uuid, special_id, order_num + count],
        )?;
        // also maybe in the future we have a "dateupdated" field or something that we also update here with a chrono::Local::now()
        conn.execute("UPDATE metadata SET songcount = songcount + 1 totaltime = totaltime + (SELECT length FROM main WHERE uniqueid = ?) WHERE playlist_id = ?", params![special_id, playlist_uuid])?;
    }
    conn.close().map_err(|(_, err)| err)?;
    info!("added to playlist successfully!");
    Ok(())
}

pub fn add_to_playlist(playlist_uuid: &str, uniqueid: &str) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    let mut count_stmt = conn
        .prepare("SELECT COUNT(*) FROM playlist_relations WHERE playlist_id = ?")
        .unwrap();
    let count = count_stmt.query_row([playlist_uuid], |row| row.get::<_, i16>(0))?;
    // drop it bcs it was holding conn
    drop(count_stmt);
    conn.execute(
        "
        INSERT INTO playlist_relations (playlist_id, song_id, user_playlist_order) VALUES (?1, ?2, ?3);
        ",
        params![&playlist_uuid, &uniqueid, count],
    )
    .unwrap();
    conn.execute("
        UPDATE metadata SET songcount = songcount + 1, totaltime = totaltime + (SELECT length FROM main WHERE uniqueid = ?) WHERE playlist_id = ?
        ", params![uniqueid, playlist_uuid]).unwrap();
    // also maybe in the future we have a "dateupdated" field or something that we also update here with a chrono::Local::now()
    conn.close().map_err(|(_, err)| err)?;
    info!("added to playlist successfully!");
    Ok(())
}

pub fn add_to_playlist_silent(playlist_uuid: &str, uniqueid: &str, count: usize) {
    // this is only called from duplicating a playlist.
    // usually metadata is handled by add_to_playlist. but in playlist duplication, we already copy that data over. so this function makes it so we aren't
    // taking the length (for example) and then adding the len of each song, effectively doubling the metadata.
    // oh, also `count`'s number is completely handled by duplicate_playlist
    let conn = Connection::open("main.db").unwrap();
    conn.execute(
            "
        INSERT INTO playlist_relations (playlist_id, song_id, user_playlist_order) VALUES (?1, ?2, ?3)",
            params![&playlist_uuid, uniqueid, count],
        ).unwrap();
    conn.close().map_err(|(_, err)| err).unwrap();
}
