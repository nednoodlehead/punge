// this is a file for converting time types for use in the player, and the resulting player
// also, this commit should change the data type of "length" in "main" table
use crate::types::DatabaseErrors;
use crate::types::PungeMusicObject;
use chrono::NaiveDate;

pub fn sec_to_time(mut int: usize) -> String {
    // format: HH:MM:SS
    // hours are a special case, if there aren't any
    let (mut sec, mut min, mut hour) = (0, 0, 0);
    while int != 0 {
        sec += 1;
        if sec == 60 {
            min += 1
        }
        if min == 60 {
            hour += 1
        }
        int -= 1;
    }
    if hour == 0 {
        format!("{}:{}", min, sec)
    } else {
        format!("{}:{}:{}", hour, min, sec)
    }
}

pub fn legacy_old_time_to_new() -> Result<(), DatabaseErrors> {
    let all = get_all_main().unwrap();
    let mut new: Vec<PungeMusicObject> = Vec::new();

    for x in all {
        new.push(PungeMusicObject {
            title: x.title,
            author: x.author,
            album: x.album,
            features: x.features,
            savelocationmp3: x.savelocationmp3,
            savelocationjpg: x.savelocationjpg,
            length: time_to_sec(&x.length),
            datedownloaded: x.datedownloaded,
            lastlistenedto: x.lastlistenedto,
            ischild: x.ischild,
            uniqueid: x.uniqueid,
            plays: x.plays,
            weight: x.weight,
            threshold: x.threshold,
        });
        crate::db::create_db::create_table_defaults().unwrap();
        for y in new.clone().into_iter() {
            add_to_main(y).unwrap();
        }
    }

    Ok(())
}

pub fn time_to_sec(time: &str) -> usize {
    // should all be HH:MM:SS
    let times: Vec<&str> = time.split(":").collect();
    let mut val: usize = 0;
    for (count, x) in times.iter().enumerate() {
        let num = x.parse::<usize>().unwrap();
        if count == 0 {
            val += num; // second :D
        }
        if count == 1 {
            val += num * 60 // minute
        }
        if count == 2 {
            val += num * 3600 // hour
        }
    }
    val
}

pub fn add_to_main(music_obj: PungeMusicObject) -> Result<String, DatabaseErrors> {
    let conn = rusqlite::Connection::open("main.db")?;
    conn.execute("INSERT INTO main (title, author, album, features, length, savelocationmp3,\
                    savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays, weight, threshold)\
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                 rusqlite::params![music_obj.title, music_obj.author, music_obj.album, music_obj.features, music_obj.length, music_obj.savelocationmp3,
                 music_obj.savelocationjpg, music_obj.datedownloaded, music_obj.lastlistenedto, music_obj.ischild, music_obj.uniqueid,
                 music_obj.plays, music_obj.weight, music_obj.threshold])?;
    conn.close().map_err(|(_, err)| err)?;
    Ok(format!("{} - {}", &music_obj.title, &music_obj.author))
}

#[derive(Clone)]
pub struct OldPungeMusicObject {
    pub title: String,
    pub author: String,
    pub album: String,
    pub features: String,
    pub length: String, // in seconds
    pub savelocationmp3: String,
    pub savelocationjpg: String,
    pub datedownloaded: NaiveDate,
    pub lastlistenedto: NaiveDate,
    pub ischild: bool, // used in reconstruction of lost music that exists in DB
    pub uniqueid: String,
    pub plays: u16,
    pub weight: i16,
    pub threshold: u16,
}

pub fn get_all_main() -> Result<Vec<OldPungeMusicObject>, DatabaseErrors> {
    // erm, forgot to close the db connection :nerd:
    let conn = rusqlite::Connection::open("main_OLD.db")?;
    let mut ret_vec: Vec<OldPungeMusicObject> = Vec::new();
    let mut stmt = conn.prepare("SELECT title, author, album, features,
    length, savelocationmp3, savelocationjpg, datedownloaded, lastlistenedto, ischild, uniqueid, plays,
    weight, threshold FROM main")?;
    let song_iter = stmt.query_map(rusqlite::params![], |row| {
        Ok(OldPungeMusicObject {
            title: row.get(0)?,
            author: row.get(1)?,
            album: row.get(2)?,
            features: row.get(3)?,
            length: row.get(4)?,
            savelocationmp3: row.get(5)?,
            savelocationjpg: row.get(6)?,
            datedownloaded: row.get(7)?,
            lastlistenedto: row.get(8)?,
            ischild: row.get(9)?,
            uniqueid: row.get(10)?,
            plays: row.get(11)?,
            weight: row.get(12)?,
            threshold: row.get(13)?,
        })
    })?;
    for obj in song_iter {
        ret_vec.push(obj.unwrap())
    }
    Ok(ret_vec)
}
