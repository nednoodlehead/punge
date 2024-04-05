// in this file, we are adding a new field to our db "threshold", which is calculated from existing values
// the purpose is for our src\gui\subscription.rs database subscription to read a value from db (instead of doing an unneeded calculation to get it)

use crate::types::PungeMusicObject;

use rusqlite::{params, Connection};

// pub fn convert() {
//     let conn = Connection::open("main.db").unwrap();
//     let mut stmt = conn.prepare("SELECT * FROM main").unwrap();
//     let _new_items: Vec<PungeMusicObject> = Vec::new();
//     let obj_iter = stmt
//         .query_map([], |row| {
//             Ok(PungeMusicObject {
//                 title: row.get(0).unwrap(),
//                 author: row.get(1).unwrap(),
//                 album: row.get(2).unwrap(),
//                 features: row.get(3).unwrap(),
//                 length: row.get(4).unwrap(),
//                 savelocationmp3: row.get(5).unwrap(),
//                 savelocationjpg: row.get(6).unwrap(),
//                 datedownloaded: row.get(7).unwrap(),
//                 lastlistenedto: row.get(8).unwrap(),
//                 ischild: row.get(9).unwrap(),
//                 uniqueid: row.get(10).unwrap(),
//                 plays: row.get(11).unwrap(),
//                 weight: row.get(12).unwrap(),
//                 threshold: calc_thres(row.get(4).unwrap()),
//             })
//         })
//         .unwrap();
//     let stmt2 = "ALTER TABLE main ADD threshold SMALLINT";
//     conn.execute(stmt2, []).unwrap();
//     let stmt_3 = "UPDATE main SET threshold = ?1 WHERE uniqueid = ?";
//     for item in obj_iter {
//         let new_item = item.unwrap();
//         println!("updating: {}", &new_item.title);
//         conn.execute(
//             stmt_3,
//             params![calc_thres(new_item.length as u16), new_item.uniqueid],
//         )
//         .unwrap();
//     }
//     println!("it worked :P");
// }
// old, keep around for now..
pub fn _calc_thres(len: String) -> u16 {
    // len: 00:10:12 format
    let sep: Vec<&str> = len.split(':').collect();
    // nothing reaches this in the db lol let hrs = sep[0].parse::<u16>().unwrap();
    let min = sep[1].parse::<u16>().unwrap();
    let sec = sep[2].parse::<u16>().unwrap();
    let total = (min * 60) + sec;
    if total / 15 == 0 {
        // so if the song is sub 15 seconds, we have the case for that
        0
    } else {
        (total / 15) - 1 // purpose is for punge to be able to detect (using gui\subscription::databasesub_2 rn name) in the case where the first 15 seconds is not detected because of the random
                         // 15 second timer, and if it started right at the end of the first song
    }
}

pub fn calc_thres(len: usize) -> usize {
    if len / 15 == 0 {
        0
    } else {
        ((len / 15) - 1).into()
    }
}
