// this file is used for making connection to the database and updating metadata for songs.
// supposedly, making these connections and closing them has almost no overhead, and since they get called, it is no big deal. I dont even see a reason to put this on a subsccription.
// maybe depending on if we are doing calculations to determine values, that takes more than like 0.001 seconds, we can use a subscription. not for now though
use crate::playliststructs::AppError;
use rusqlite::{params, Connection};
/*
song is played fully and naturally transposes to the next song
    plays += 1;
    weight += 2;


    */

// realistically, we dont even need this to return a result, since the only place this will be called, it will just .unwrap()
// bcs there is nothing to do with the result (like returning info to user) and if it fails, the program should panic
// something something best practise (i also like using ? :3)

// yeah on debug mode, this is reallllllly slow (like 0.5seconds)
// edit: now runs on a subscription, it is much faster :D

// for times when the player just autoplays to the next song, user liked it enough to let it play
pub fn on_passive_play(uniqueid: String) -> Result<(), AppError> {
    let conn = Connection::open("main.db")?;
    let stmt = "UPDATE main SET plays = plays +1, weight = weight + 2 WHERE uniqueid = ?";
    conn.execute(stmt, params![uniqueid])?;
    conn.close().map_err(|(_, err)| err)?; // we dont need the connection
    Ok(())
}
// for times when the user seeks out the song. add more to weight, since user probably likes this song
pub fn on_seek(uniqueid: String) -> Result<(), AppError> {
    let conn = Connection::open("main.db")?;
    let stmt = "UPDATE main SET weight = weight + 5 WHERE uniqueid = ?";
    conn.execute(stmt, params![uniqueid])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

// i think it makes sense if this is only for forward skips. so its like "no i dont want that one", backwards skip is more like "i know what song i want and am going back to it"
pub fn skipped_song(uniqueid: String) -> Result<(), AppError> {
    let conn = Connection::open("main.db")?;
    let stmt = "UPDATE main SET weight = weight -1 WHERE uniqueid =?";
    conn.execute(stmt, params![uniqueid])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

// new ones for new data collection

pub fn add_one_weight(uniqueid: String) -> Result<(), AppError> {
    let conn = Connection::open("main.db")?;
    let stmt = "UPDATE main SET weight = weight +1 WHERE uniqueid = ?";
    conn.execute(stmt, params![uniqueid])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
pub fn add_one_play(uniqueid: String) -> Result<(), AppError> {
    let conn = Connection::open("main.db")?;
    let stmt = "UPDATE main SET plays = plays +1 WHERE uniqueid = ?";
    conn.execute(stmt, params![uniqueid])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
