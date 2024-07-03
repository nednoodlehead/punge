use crate::types::{DatabaseErrors, PungeMusicObject, UserPlaylist};
use log::info;
use rusqlite::{params, Connection};

pub fn add_to_main(music_obj: PungeMusicObject) -> Result<String, DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    info!("Adding into main!");
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
    info!("inserting, playlist does not exist");
    conn.execute(
        "INSERT INTO metadata (title, description, thumbnail, datecreated,\
        songcount, totaltime, isautogen, userorder, playlist_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            new_playlist.title,
            new_playlist.description,
            new_playlist.thumbnail,
            new_playlist.datecreated,
            new_playlist.songcount,
            new_playlist.totaltime,
            new_playlist.isautogen,
            new_playlist.userorder,
            new_playlist.uniqueid
        ],
    )?;

    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn add_to_playlist(playlist_uuid: &str, uniqueid: Vec<String>) -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    for special_id in uniqueid.iter() {
        conn.execute(
            "INSERT INTO playlist_relations (playlist_id, song_id) VALUES (?1, ?2)",
            params![playlist_uuid, special_id],
        )?;
    }
    conn.close().map_err(|(_, err)| err)?;
    info!("added to playlist successfully!");
    Ok(())
}

pub fn add_empty_entries(uuids: Vec<String>) -> Result<(), DatabaseErrors> {
    // we add dummy data so when a playlist is made, and the playlist is refreshed in-app,
    // it doesn't throw errors for null columns
    let conn = Connection::open("main.db")?;
    for str in uuids {
        conn.execute("INSERT INTO main (title, author, album, features, length, savelocationmp3,\
                    savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays, weight, threshold)\
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)", params!["placeholder_title", "placeholder_author", "placeholder_album", "placeholder_features",
            20, "placeholder_mp3", "placeholder_jpg", chrono::Local::now(), chrono::Local::now(), false, str, 0, 0, 2])?;
    }
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
