use crate::playliststructs::DatabaseErrors;
use rusqlite::{params, Connection, Error};

pub fn update_playlist(
    old_title: &str,
    new_title: &str,
    description: &str,
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
    author: &str,
    title: &str,
    album: &str,
    features: &str,
    unique: &str,
) -> Result<(), DatabaseErrors> {
    let conn: Connection = rusqlite::Connection::open("main.db")?;
    let statement: &str =
        "UPDATE main author = ?, title = ?, album = ?, features = ? WHERE uniqueid = ?";
    conn.execute(statement, params![author, title, album, features])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}

pub fn quick_swap_title_author(
    author: &str,
    title: &str,
    uniqueid: &str,
) -> Result<(), DatabaseErrors> {
    let conn: Connection = rusqlite::Connection::open("main.db")?;
    let statement: &str = "UPDATE main author = ?, title = ? WHERE uniqueid = ?";
    conn.execute(statement, params![title, author, uniqueid])?;
    // conn.close() returns an err and connection. We drop the connection with .map_err()
    conn.close().map_err(|(_, err)| err)?;
    Ok(())
}
