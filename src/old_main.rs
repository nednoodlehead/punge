// #[path = "./utils/decide_youtube.rs"]
// mod decide;
// use crate::decide::download;

#[path = "./utils/youtube_interface.rs"]
mod yt;
use crate::yt::{download};

#[path = "db/create_db.rs"]
mod create_db;
// calls the PungeMusicObject into scope
mod playliststructs;

use crate::create_db::create_table_defaults;

use crate::playliststructs::{Playlist, PungeMusicObject, UserPlaylist};
use rusqlite::{params, Connection, Params};

#[path = "./db/insert.rs"]
mod insert;

#[path = "./db/fetch.rs"]
mod fetch;

use chrono::{Local, NaiveDate};

#[path = "./db/update.rs"]
mod update;

#[path = "./utils/sep_video.rs"]
mod sep;

#[path = "./utils/decide_youtube.rs"]
mod dec;

use crate::fetch::get_all_from_playlist;
use crate::insert::add_to_playlist;
use uuid::Uuid;

use itertools::Itertools;
use crate::dec::fetch_stragglers;
#[path = "player/interface.rs"]
mod interface;

// https://www.youtube.com/watch?v=GgkV01XE8mQ
fn main() {
    // let power = "https://www.youtube.com/watch?v=chPDTUjnWgA";
    // let car_missin_530 = "https://www.youtube.com/watch?v=uRw5IE8HXkE";
   // let animals = "https://www.youtube.com/watch?v=nRkUR_GdCy4&list=OLAK5uy_lSqtXtMNNoNWl28J33mzcT8NSqZIzHsu8";
   // let tlop = "https://www.youtube.com/watch?v=6oHdAA3AqnE&list=PLzMq4yH_FvVac_1R0DMcMkcwnJ1-hFx6b";
   // let basement_demos = "https://www.youtube.com/watch?v=adHiXJZxiY4";  // one video = album
   fetch::get_uuid_from_name("my awesome title".to_string());
}
