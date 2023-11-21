// use crate::playliststructs::PungeMusicObject;
// use crate::utils::db_convert::convert_duration_format;
// use crate::utils::db_convert::OldData;
// use crate::utils::decide_youtube::clean_inputs_for_win_saving;
// use chrono::Local;
// use std::process::Command;

// // converting pytube downloaded mp3s -> wav then back to mp3s, might solve the rodio issue
// // new_path_base must end with a /
// pub fn convert_mp3_to_mp3(old_obj: OldData, new_path_base: String) -> PungeMusicObject {
//     let clean_auth = clean_inputs_for_win_saving(old_obj.author.clone());
//     let clean_title = clean_inputs_for_win_saving(old_obj.title.clone());

//     let proper_path = format!("{}{} - {}.mp3", new_path_base, clean_auth, clean_title);
//     let temp_wav = format!("f:/{}.wav", old_obj.uniqueid.clone());
//     let first_ffmpeg_args = &[
//         "-i",
//         old_obj.savelocation.as_str(),
//         "-vn",
//         temp_wav.as_str(),
//     ];

//     let mut ffmpeg_1 = Command::new("ffmpeg.exe").args(first_ffmpeg_args).output();
//     println!("ffmpeg_1 output {:?}", ffmpeg_1.unwrap());

//     let second_ffmpeg_args = &[
//         "-i",
//         temp_wav.as_str(),
//         "-codec:a",
//         "libmp3lame",
//         "-q:a",
//         "0",
//         proper_path.as_str(),
//     ];
//     println!("proper path: {}", &proper_path);

//     let ffmpeg_2 = Command::new("ffmpeg.exe").args(second_ffmpeg_args).output();
//     println!("ffmpeg_2 output {:?}", ffmpeg_2.unwrap());
//     std::fs::remove_file(temp_wav).unwrap();

//     PungeMusicObject {
//         title: old_obj.title.to_string(),
//         author: old_obj.author.to_string(),
//         album: old_obj.album,
//         features: "none".to_string(),
//         length: convert_duration_format(proper_path.clone()),
//         savelocationmp3: proper_path,
//         savelocationjpg: old_obj.savelocationthumb,
//         datedownloaded: Local::now().date_naive(),
//         lastlistenedto: Local::now().date_naive(),
//         ischild: false,
//         uniqueid: old_obj.uniqueid,
//         plays: 0,
//         weight: 0,
//         threshold: 0, // idc this wont be used anymore
//     }
// }

// pub fn find_and_convert_old_db() {
//     let missing_entries = crate::utils::compare_db::find_missing();
//     for missing in missing_entries {
//         let base_path = r"F:\Projects\Python Projects\punge\default\mp3\".to_string();
//         let new_obj = convert_mp3_to_mp3(missing, base_path);
//         match crate::db::insert::add_to_main(new_obj) {
//             Ok(t) => {
//                 println!("Success insert: {}", t)
//             }
//             Err(e) => {
//                 println!("epic database fail: {:?}", e)
//             }
//         }
//     }
// }
