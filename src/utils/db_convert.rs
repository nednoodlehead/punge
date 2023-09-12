use crate::playliststructs::{DatabaseErrors, PungeMusicObject};
use chrono::Local;
use mp3_duration;
use rodio::Source;
use rusqlite::{params, Connection, Result};
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

struct Link {
    title: String,
    youtube: String,
}
// ok, this doesnt work bcs the old punge downloaded mp3s cannot be opened. codeword: rodio cant open them,
// and they cannot be opened to see length. will make a new helper function to redownload all of them
pub fn convert_and_insert(old_db_path: String) -> Result<(), DatabaseErrors> {
    let new_db_path = "f:/projects/Python Projects/Punge";
    let conn = Connection::open(r"f:/punge releases/punge_newest_2/Punge/MAINPLAYLIST.sqlite")?;
    let mut stmt = conn.prepare(
        "SELECT title, author, savelocation, savelocationthumb, album, uniqueid FROM main",
    )?;
    let old_obj_iter = stmt.query_map([], |row| {
        Ok(OldData {
            title: row.get(0)?,
            author: row.get(1)?,
            savelocation: row.get(2)?,
            savelocationthumb: row.get(3)?,
            album: row.get(4)?,
            uniqueid: row.get(5)?,
        })
    })?;
    for prior_obj in old_obj_iter {
        let old_obj = prior_obj.unwrap();
        let new_path = format!(
            r"F:\Projects\Python Projects\punge\default\mp3\{}",
            &old_obj.savelocation[27..]
        );

        println!("old path: {}", &old_obj.savelocation);
        let new_obj = PungeMusicObject {
            title: old_obj.title,
            author: old_obj.author,
            album: old_obj.album,
            features: String::from("None"),
            length: convert_duration_format(old_obj.savelocation.clone()),
            savelocationmp3: new_path,
            savelocationjpg: old_obj.savelocationthumb,
            datedownloaded: Local::now().date_naive(),
            lastlistenedto: Local::now().date_naive(),
            ischild: false,
            uniqueid: old_obj.uniqueid,
            plays: 0,
            weight: 0,
        };
        println!("OBJECT: {:?}", new_obj);
    }
    Ok(())
}

fn get_duration(path: String) -> std::time::Duration {
    let file = Path::new(path.as_str());
    mp3_duration::from_path(&file).unwrap()
}

pub fn convert_duration_format(path: String) -> String {
    let total_seconds = get_duration(path).as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

// fn redownload_from_old_punge() {
//     let old_path = r"F:\punge releases\punge_newest_2\Punge\MAINPLAYLIST.sqlite";
//     let conn = Connection::open(old_path).unwrap();
//     let mut stmt = conn.prepare("SELECT Title, Author, Savelocation
//         , SavelocationThumb, Album, Uniqueid from MAIN").unwrap();
//     let db_iter = stmt.query_map(params![], |row| {
//         Ok(OldData { title: row.get(0).unwrap(),
//          author: row.get(1).unwrap(),
//          savelocation: row.get(2).unwrap(),
//           savelocationthumb: row.get(3).unwrap(),
//          album: row.get(4),
//          uniqueid: row.get(5) }
//         }).unwrap();
//         let mut temp_check: Vec<OldData> = Vec::new();
//         let check_bool = false;
//         // at least one of the downloaded videos end with a 0, a comprehensive check is done for album songs
//         // btw this design pattern is fixed with 'is_child' field in new db
//         for obj in db_iter() {
//             if obj.uniqueid.ends_with(temp_check.len() as &str) {
//                 temp_check.push(obj);
//             }
//             else {
//                 for pot_download in download()
//                 match download(format!("www.youtube.com/?v={}", obj.uniqueid.clone())) {
//                     Ok(t)
//                 }
//             }
//         }
//     } )
// }

// create new function that is just like utils::youtube_interfacee::download() but doesnt need the gui at all
// so no worry about thread

use crate::playliststructs::{AppError, Playlist};
use crate::utils::decide_youtube::{begin_playlist, begin_single};
use crate::utils::youtube_interface::{check_if_exists, playlist_parse, single_parse};
pub fn download(link: String) -> Vec<Result<(String, String), AppError>> {
    // (String, String) = (url, auth and title)

    let mut values: Vec<Result<(String, String), AppError>> = vec![];
    let link_clone = link.clone();
    if link.contains("list=") {
        let vid = playlist_parse(link);
        match vid {
            Ok(ok_list) => {
                //  match begin_playlist(ok_list) {
                for item in begin_playlist(ok_list) {
                    match item {
                        Ok(good_vid) => {
                            values.push(Ok((link_clone.clone(), good_vid)));
                        }
                        Err(e) => {
                            values.push(Err((e)));
                        }
                    }
                }
                // }
            }
            Err(e) => {
                values.push(Err(e));
            }
        }
    } else {
        let vid = single_parse(link);
        match vid {
            Ok(video) => {
                for item in begin_single(video) {
                    match item {
                        Ok(good_vid) => {
                            values.push(Ok((link_clone.clone(), good_vid)));
                        }
                        Err(e) => {
                            values.push(Err(e));
                        }
                    }
                }
            }
            Err(e) => {
                values.push(Err(e));
            }
        }
    }
    values
}

pub fn actually_download_old_songs() {
    let old_path = r"F:\punge releases\punge_newest_2\Punge\MAINPLAYLIST.sqlite";
    let conn = Connection::open(old_path).unwrap();
    let mut stmt = conn
        .prepare("SELECT Title, Author, Uniqueid from MAIN")
        .unwrap();
    let db_iter = stmt
        .query_map(params![], |row| {
            Ok(Link {
                title: row.get(0).unwrap(),
                youtube: row.get(2).unwrap(),
            })
        })
        .unwrap();
    let mut holding = vec![];
    let mut bool_check = false;
    let mut downloaded_special: Vec<String> = vec![];
    for old_entry in db_iter {
        let mut entry = old_entry.unwrap().youtube;
        let last_char = entry.chars().last();
        let last_char_digit = last_char.map(|c| c.to_digit(10)).flatten();
        if last_char_digit == Some(holding.len() as u32) {
            if last_char_digit == Some(0) {
                // trunicate
                let entry = if entry.len() == 11 {
                    entry
                } else if entry.len() == 12 {
                    entry[..entry.len() - 1].to_owned()
                } else {
                    entry[..entry.len() - 2].to_owned()
                };
                println!("first instance: https://www.youtube.com/watch?v={}", entry);
                bool_check = true;
                holding.push(entry.clone());
            } else {
                if last_char_digit == Some(holding.len() as u32) {
                    // ignoring from one video
                    println!("skipping {}", &entry);
                    holding.push(entry.clone());
                } else {
                    // we have broken out of the loop, download normally
                    println!(
                        "download (no more loop): https://www.youtube.com/watch?v={}",
                        entry
                    );
                    holding.clear() // now that the video set is gone, #ignore
                }
            }
        } else {
            println!(
                "average download: https://www.youtube.com/watch?v={} | len={}",
                entry,
                holding.len()
            );
            holding.clear();
        }
    } // for loop
}
