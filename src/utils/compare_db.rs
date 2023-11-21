// use rusqlite::Connection;

// use crate::playliststructs::{DatabaseErrors, PungeMusicObject};
// use crate::utils::db_convert::OldData; // old db // new db

// use crate::db::fetch;

// pub fn get_old_db() -> Result<Vec<OldData>, DatabaseErrors> {
//     let conn = Connection::open(r"f:/punge releases/punge_newest_2/Punge/MAINPLAYLIST.sqlite")?;
//     let mut stmt = conn.prepare(
//         "SELECT title, author, savelocation, savelocationthumb, album,
//         uniqueid FROM main",
//     )?;
//     let old_obj_iter = stmt.query_map([], |row| {
//         Ok(OldData {
//             title: row.get(0)?,
//             author: row.get(1)?,
//             savelocation: row.get(2)?,
//             savelocationthumb: row.get(3)?,
//             album: row.get(4)?,
//             uniqueid: row.get(5)?,
//         })
//     })?;
//     let mut old_data_vec: Vec<OldData> = vec![];
//     for item in old_obj_iter {
//         old_data_vec.push(item.unwrap());
//     }
//     Ok(old_data_vec)
// }

// pub fn find_missing() -> Vec<OldData> {
//     let old_db = get_old_db().unwrap();
//     let new_db = fetch::get_all_main().unwrap();
//     let mut excluded: Vec<OldData> = Vec::new();
//     let mut ids: Vec<String> = vec![];
//     for item in new_db {
//         ids.push(item.uniqueid.clone())
//     }
//     for old_entry in old_db {
//         if !ids.contains(&old_entry.uniqueid) {
//             excluded.push(old_entry);
//         }
//     }
//     excluded
// }

// // use crate::utils::db_convert::download;

// pub fn download_missing() {
//     for missing in find_missing() {
//         let real_id = if missing.uniqueid.len() == 11 {
//             missing.uniqueid
//         } else if missing.uniqueid.len() == 12 {
//             "bruh again".to_string()
//         } else {
//             "bruh".to_string() // ignore and let it fail
//         };
//         let link = format!("https://www.youtube.com/watch?v={}", real_id);
//         println!("Trying to download: {}", &link);
//         // println!("download: {:?}", download(link));
//     }
// }

// use std::collections::HashMap;
// pub fn reorder_db() {
//     let new_data = fetch::get_all_main().unwrap();
//     let old_data = get_old_db().unwrap();
//     let mut data_struct: Vec<(String, Option<PungeMusicObject>)> = Vec::new(); // (old_data.uniqueid, new_data)
//     for data in old_data {
//         data_struct.push((data.uniqueid.clone(), None))
//     }
//     for new_entry in new_data {
//         for mut existing_entry in data_struct.iter_mut() {
//             if existing_entry.0 == new_entry.uniqueid {
//                 existing_entry.1 = Some(new_entry.clone())
//             }
//         }
//     }
//     for entry in data_struct {
//         println!("{:?}", entry);
//     }
// }
