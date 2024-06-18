// this file contains all of the logic and data related to downloading and processing images
// thumbnails will be trimmed down to 1:1 ratio so they fit in the audio square nicely
// in theory these images will come from the youtube thumbnail, then are processed
use reqwest;
// returns the path to the downloaded thumbnail
pub fn _get_raw_thumbnail_from_link(youtube_id: &str, dir_path: &str) -> Result<String, ()> {
    // https://youtube.com/watch?v=NPqDIwWMtxg
    // turns into: https://img.youtube.com/vi/NPqDIwWMtxg/maxresdefault.jpg
    let base_url = format!(
        "https://img.youtube.com/vi/{}/maxresdefault.jpg",
        youtube_id
    );
    let file_path: String = format!("{}{}.png", dir_path, youtube_id);
    // check if it is downloaded?!
    if std::path::Path::exists(&std::path::Path::new(&file_path)) {
        return Err(());
    }
    let mut file = std::fs::File::create(&file_path).unwrap();
    reqwest::blocking::get(base_url)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();

    Ok(file_path)
}

// overarching concept:
// images are stored in one of two places
// some temp folder: searched images and whatnot
// ./default/jpg/ for thumbnails that are used for videos!

// pub fn fetch_and_crop_image(youtube_id: &str, dir_path: &str) -> Result<String, ()> {
//     match get_raw_thumbnail_from_link(youtube_id, dir_path) {
//         // new path just dropped
//         Ok(path) => {
//             //jashdj
//         }
//         // the path exists, do we crop ... ?
//         Err(_) => {}
//     }
// }
