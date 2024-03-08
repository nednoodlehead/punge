// // purpose of this file is to download youtube videos and name them appropriately. it calls
// // various functions from "decide_youtube.rs" to determine the title, author and album

// use rusqlite;
// use rusqlite::params;

// // #[path = "./decide_youtube.rs"]
// // mod decide;
// // use decide::entry;

// use crate::types::{AppError, Playlist};
// use crate::utils::decide_youtube::{begin_playlist, begin_single};

// // this is the function exposed to the rest of the app. It takes in the youtube link

// pub async fn download(link: String) -> Vec<Result<(String, String), AppError>> {
//     // (String, String) = (url, auth and title)

//     let mut values: Vec<Result<(String, String), AppError>> = vec![];
//     let link_clone = link.clone();
//     if link.contains("list=") {
//         let vid = playlist_parse(link);
//         match vid {
//             Ok(ok_list) => {
//                 //  match begin_playlist(ok_list) {
//                 for item in begin_playlist(ok_list).await {
//                     match item {
//                         Ok(good_vid) => {
//                             values.push(Ok((link_clone.clone(), good_vid)));
//                         }
//                         Err(e) => {
//                             values.push(Err(e));
//                         }
//                     }
//                 }
//                 // }
//             }
//             Err(e) => {
//                 values.push(Err(e));
//             }
//         }
//     } else {
//         let vid = single_parse(link);
//         match vid {
//             Ok(video) => {
//                 for item in begin_single(video).await {
//                     match item {
//                         Ok(good_vid) => {
//                             values.push(Ok((link_clone.clone(), good_vid)));
//                         }
//                         Err(e) => {
//                             values.push(Err(e));
//                         }
//                     }
//                 }
//             }
//             Err(e) => {
//                 values.push(Err(e));
//             }
//         }
//     }
//     values
// }

// pub fn check_if_exists(uniqueid: String) {
//     // checks if the given unique id is found inside the main table. aka: has it been downloaded?
//     let conn = rusqlite::Connection::open("main.db").unwrap();
//     let res: String = conn
//         .query_row(
//             "SELECT EXISTS(SELECT 1 FROM main WHERE uniqueid=?)",
//             params![uniqueid.as_str()],
//             |row| row.get(0),
//         )
//         .unwrap();
//     println!("res: {}", res)
// }

// pub fn playlist_parse(playlist_link: String) -> Result<Playlist, AppError> {
//     let playlist: Result<Playlist, AppError> =
//         crate::utils::playlist::get_playlist(playlist_link.as_str());
//     match playlist {
//         Ok(good_playlist) => Ok(good_playlist),
//         Err(e) => Err(AppError::InvalidUrlError(format!("{:?}", e))),
//     }
// }

// pub fn single_parse(single_link: String) -> Result<rustube::blocking::video::Video, AppError> {
//     // create a url type, used to create the video type
//     let url = rustube::url::Url::parse(single_link.as_str());
//     match url {
//         Ok(good_url) => {
//             let vid = rustube::blocking::Video::from_url(&good_url)?; // fails here if video is unavailable
//             Ok(vid)
//         }
//         Err(e) => Err(AppError::InvalidUrlError(e.to_string())),
//     }
// }
