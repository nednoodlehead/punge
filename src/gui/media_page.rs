use crate::gui::messages::{Page, ProgramCommands};
use crate::gui::persistent;
use crate::types::AppError;
use rusty_ytdl;

use iced::widget::{container, text};
use iced::Element;

pub struct MediaPage {
    pub download_input: String,
    pub download_to_location: String, // we can read from json at some point
    pub download_feedback: Vec<String>,
}
impl MediaPage {
    pub fn new() -> Self {
        MediaPage {
            download_input: "".to_string(),
            download_to_location: "".to_string(),
            download_feedback: Vec::new(),
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        container(persistent::render_top_buttons(Page::Media)).into()
    }
}

// should have option to choose mp4 vs mp3
pub async fn download_content(link: String, download_path: String) -> Result<String, AppError> {
    // string returned is what the user will see, either an error or download success
    // i dont really see a reason to be able to download playlists like this. who wants to download videos in batch like that? idk idc
    if link.contains("youtube") {
        download_youtube(link, download_path).await?;
    } else if link.contains("instagram") {
        _download_insta();
    } else {
        return Err(AppError::InvalidUrlError(
            "Link does not contain 'instagram' or 'youtube'".to_string(),
        ));
    }
    todo!()
}

// TODO (at some point), make this async and have it send o the subscription that is listening
// for youtube events. not sure how that will be handled for instagram download...
async fn download_youtube(link: String, mut path: String) -> Result<String, AppError> {
    let vid = rusty_ytdl::Video::new(link)?;
    // make the path end with a slash
    path = if !path.ends_with("\\") | !path.ends_with("/") {
        format!("{}/", path)
    } else {
        path
    };
    let title = vid.get_basic_info().await?.video_details.title;
    let full_output = format!("{}{} - {}", path, &title, vid.get_video_url());
    let new_path = std::path::Path::new(&full_output);
    vid.download(new_path).await?;
    Ok(format!("{} downloaded successfully!", title))
}

fn _download_insta() {}
