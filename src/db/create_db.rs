use crate::types::DatabaseErrors;
use chrono::Local;
use rusqlite::{params, Connection};
// Creates the file with the two default tables :D

pub fn create_table_defaults() -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "CREATE TABLE main (
            title TEXT,
            author TEXT,
            album TEXT,
            features TEXT,
            length SMALLINT,
            savelocationmp3 TEXT,
            savelocationjpg TEXT,
            datedownloaded DATE,
            lastlistenedto DATE,
            ischild BOOL,
            uniqueid TEXT PRIMARY KEY,
            plays SMALLINT,
            weight SMALLINT,
            threshold SMALLINT
            )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE metadata (
            title TEXT,
            description TEXT,
            thumbnail TEXT,
            datecreated DATE,
            songcount SMALLINT,
            totaltime TEXT,
            isautogen BOOL,
            userorder SMALLINT,
            playlist_id TEXT
            )",
        params![],
    )?;
    conn.execute(
        "CREATE TABLE playlist_relations (
            playlist_id TEXT,
            song_id TEXT
            )",
        params![],
    )?;
    conn.execute(
        // the main 'playlist' :)
        "INSERT INTO metadata (title, description, thumbnail, datecreated,
        songcount, totaltime, isautogen, userorder, playlist_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            String::from("Main"),
            String::from("All of your music"),
            String::from("No thumbnail"),
            Local::now().date_naive(),
            0,
            String::from("00:00:00"),
            false, // so technically it is 'auto gen', but not in the right sense
            1,
            "main"
        ],
    )?;
    Ok(())
}
