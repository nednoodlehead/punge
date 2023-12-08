use crate::gui::messages::ProgramCommands;


use iced::widget::{container, text};
use iced::{Element};

pub struct MediaPage {
    pub download_input: String,
    pub download_to_location: String, // we can read from json at some point
    pub download_feedback: String,
}
impl MediaPage {
    pub fn new() -> Self {
        MediaPage {
            download_input: "".to_string(),
            download_to_location: "".to_string(),
            download_feedback: "".to_string(),
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        container(text("here")).into()
    }
}

// will need to be a subscription at some point? async too
fn download_content(link: String, download_path: String) -> String {
    // string returned is what the user will see, either an error or download success
    // i dont really see a reason to be able to download playlists like this. who wants to download videos in batch like that? idk idc
    if link.contains("youtube") {
        download_youtube(link, download_path);
    } else if link.contains("instagram") {
        _download_insta();
    } else {
        return "Link does not contain 'instagram' or 'youtube'".to_string();
    }
    todo!()
}



// TODO (at some point), make this async and have it send o the subscription that is listening
// for youtube events. not sure how that will be handled for instagram download...
fn download_youtube(link: String, _path: String) -> String {
    let downloaded = rustube::blocking::download_best_quality(&link);
    match downloaded {
        Ok(done_path) => {
            format!("Video has been downloaded to: {:?}", done_path)
        }
        Err(e) => {
            format!("Error downloading: {:?}", e)
        }
    }
}

fn _download_insta() {}
