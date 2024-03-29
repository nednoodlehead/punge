use crate::types::{DatabaseErrors, PungeMusicObject, UserPlaylist};
use rusqlite::{params, Connection};

pub fn add_to_main(music_obj: PungeMusicObject) -> Result<String, DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute("INSERT INTO main (title, author, album, features, length, savelocationmp3,\
                    savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays, weight, threshold)\
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                 params![music_obj.title, music_obj.author, music_obj.album, music_obj.features, music_obj.length, music_obj.savelocationmp3,
                 music_obj.savelocationjpg, music_obj.datedownloaded, music_obj.lastlistenedto, music_obj.ischild, music_obj.uniqueid,
                 music_obj.plays, music_obj.weight, music_obj.threshold])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(format!("{} - {}", &music_obj.title, &music_obj.author))
}

pub fn create_playlist(new_playlist: UserPlaylist) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    // check if that table already exists
    println!("inserting, playlist does not exist");
    conn.execute(
        "INSERT INTO metadata (title, description, thumbnail, datecreated,\
        songcount, totaltime, isautogen, playlist_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            new_playlist.title,
            new_playlist.description,
            new_playlist.thumbnail,
            new_playlist.datecreated,
            new_playlist.songcount,
            new_playlist.totaltime,
            new_playlist.isautogen,
            new_playlist.uniqueid
        ],
    )?;

    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn add_to_playlist(playlist_uuid: String, uniqueid: String) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "INSERT INTO playlist_relations (playlist_id, song_id) VALUES (?1, ?2)",
        params![playlist_uuid, uniqueid],
    )?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn add_empty_entries(uuids: Vec<String>) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    for str in uuids {
        conn.execute("INSERT into main (uniqueid) VALUES (?)", params![str])?;
    }
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
