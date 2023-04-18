
use rusqlite::{Error, Connection, params};

pub fn update_playlist(old_title: &str, new_title: &str, description: &str, image: &str) -> Result<(), rusqlite::Error> {
    // updates the title, description and image
    let conn = rusqlite::Connection::open("main.db")?;
    let statement: &str = "UPDATE metadata SET title = ?, description = ?, image = ? WHERE title = ?"?;
    conn.execute(statement, params![new_title, description, image, old_title])?;
    statement: &str = format!("ALTER TABLE {} RENAME TO {}]", old_title, new_title).as_str();
    conn.execute(statement, params![])?;
    conn.close()?;
    Ok(())
}

pub fn update_song(author: &str, title: &str, album: &str, features: &str, unique: &str) -> Result<(), rusqlite::Error> {
    let conn: Connection = rusqlite::Connection::open("main.db")?;
    let statement: &str = "UPDATE main author = ?, title = ?, album = ?, features = ? WHERE uniqueid = ?";
    conn.execute(statement, params![author, title, album, features])?;
    conn.close()?;
    Ok(())
}

pub fn quick_swap_title_author(author: &str, title: &str, uniqueid: &str) -> Result<(), rusqlite::Error> {
    let conn: Connection = rusqlite::Connection::open("main.db")?;
    let statement: &str = "UPDATE main author = ?, title = ? WHERE uniqueid = ?";
    conn.execute(statement, params![title, author, uniqueid])?;
    Ok(())
}

