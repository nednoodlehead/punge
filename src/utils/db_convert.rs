use crate::playliststructs::{DatabaseErrors, PungeMusicObject};
use chrono::Local;
use mp3_duration;
use rodio::Source;
use rusqlite::{Connection, Result};
use std::io::BufReader;
use std::path::Path;
struct OldData {
    title: String,
    author: String,
    savelocation: String,
    savelocationthumb: String,
    album: String,
    uniqueid: String,
}

// fn convert_and_insert(old_db_path: String) -> Result<(), DatabaseErrors>{
//     let new_db_path = "f:/projects/Python Projects/Punge"
//     let conn_old = Connection::open(r#"f:/punge releases/punge_newest_2/Punge/MAINPLAYLIST.sqlite"#)?;
//     let mut stmt = conn.prepare("SELECT title, author, savelocation, savelocationthumb, album, uniqueid FROM main")?;
//     let old_obj_iter = stmt.query_map([], |row| {
//         Ok(OldData {
//             title: row.get(0)?,
//             author: row.get(1)?,
//             savelocation: row.get(2)?,
//             savelocationthumb: row.get(3)?,
//             album: row.get(4)?,
//             uniqueid: row.get(5)?
//         })
//     })?;
//     for old_obj in old_obj_iter {
//         let new_obj = PungeMusicObject {
//              title: old_obj.title,
//             author: old_obj.author,
//             album: old_obj.album,
//             features: String::from("None"),
//             length: get_duration(savelocation.clone()),
//             savelocationmp3: old_obj.savelocation,
//             savelocationjpg: old_obj.savelocationthumb,
//             datedownloaded: Local::now().date_naive(),
//             las
//         }
//     }
//     Ok(())
// }

pub fn get_duration(path: String) -> String {
    let file = Path::new(path.as_str());
    format!("{:?}", mp3_duration::from_path(&file).unwrap())
}
