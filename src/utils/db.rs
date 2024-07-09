// this is pretty much only for me (or other punge enthusiasts), adds the 'order' to the database rows
// since we are actually going to do #20 now...
pub fn add_count_to_all_main() {
    let conn = rusqlite::Connection::open("main.db").unwrap();
    let mut fixed: Vec<crate::types::PungeMusicObject> = Vec::new();
    let mut stmt = conn
        .prepare(
            "SELECT title, author, album, features, length, savelocationmp3, savelocationjpg,
        datedownloaded, lastlistenedto, ischild, uniqueid, plays, weight, threshold FROM main",
        )
        .unwrap();
    let the_iter = stmt
        .query_map(rusqlite::params![], |row| {
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
        })
        .unwrap();
    for (count, obj) in the_iter.enumerate() {
        let o = obj.unwrap();
        let new_item = crate::types::PungeMusicObject {
            title: o.title,
            author: o.author,
            album: o.album,
            features: o.features,
            length: o.length,
            savelocationmp3: o.savelocationmp3,
            savelocationjpg: o.savelocationjpg,
            datedownloaded: o.datedownloaded,
            lastlistenedto: o.lastlistenedto,
            ischild: o.ischild,
            uniqueid: o.uniqueid,
            plays: o.plays,
            weight: o.weight,
            threshold: o.threshold,
            order: count,
        };
        fixed.push(new_item);
    }
    conn.execute("DROP TABLE main", rusqlite::params![])
        .unwrap();
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
            threshold SMALLINT,
            user_order INT
  )",
        rusqlite::params![],
    )
    .unwrap();
    for music_obj in fixed {
        crate::db::insert::add_to_main(music_obj).unwrap();
    }
}
use chrono::NaiveDate;

#[derive(Clone)]
pub struct OldPungeMusicObject {
    pub title: String,
    pub author: String,
    pub album: String,
    pub features: String,
    pub length: u32, // in seconds
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
