use crate::playliststructs::DatabaseErrors;
use rusqlite::{params, Connection, Params};
// Creates the file with the two default tables :D

pub fn create_table_defaults() -> Result<(), DatabaseErrors> {
    let conn = Connection::open("main.db")?;
    conn.execute(
        "CREATE TABLE main (
            title TEXT,
            author TEXT,
            album TEXT,
            features TEXT,
            length TEXT,
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
    Ok(())
}
