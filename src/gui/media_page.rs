use crate::gui::messages::{Page, ProgramCommands, TextType};
use crate::gui::persistent;
use crate::types::AppError;
use crate::types::Config;
use rusty_ytdl::{self, VideoOptions};

use iced::widget::{button, column, combo_box, horizontal_space, row, text, text_input, Container};

use iced::{Alignment, Element};

pub struct MediaPage {
    pub download_input: String,
    pub download_to_location: String, // we can read from json at some point
    pub download_feedback: Vec<String>,
    pub mp3_and_4: combo_box::State<String>,
    pub download_type: String, // mp3 or mp4
}
impl MediaPage {
    pub fn new(config: &Config) -> Self {
        MediaPage {
            download_input: "".to_string(),
            download_to_location: config.media_path.clone(),
            download_feedback: Vec::new(),
            mp3_and_4: combo_box::State::new(vec![".mp3".to_string(), ".mp4".to_string()]),
            download_type: ".mp4".to_string(),
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        let buttons_and_labels = column![
            row![
                horizontal_space(),
                text("Youtube / Instagram link").width(175.0),
                text_input(&self.download_input, &self.download_input)
                    .on_input(|txt| {
                        ProgramCommands::UpdateWidgetText(TextType::Mp4DownloadInput, txt)
                    })
                    .width(500.0),
                horizontal_space(),
            ]
            .padding(10.0)
            .align_items(Alignment::Center),
            row![
                horizontal_space(),
                text("Path:").width(175.0),
                text_input(&self.download_to_location, &self.download_to_location)
                    .on_input(|txt| {
                        ProgramCommands::UpdateWidgetText(TextType::Mp4PathInput, txt)
                    })
                    .width(500),
                horizontal_space()
            ]
            .padding(10.0),
            row![
                horizontal_space(),
                combo_box(
                    &self.mp3_and_4,
                    self.download_type.as_str(),
                    None,
                    ProgramCommands::UpdateMp3Or4Combobox
                )
                .width(50.0),
                button(text("Download!"))
                    .on_press(ProgramCommands::DownloadMedia(
                        self.download_input.clone(),
                        self.download_to_location.clone(),
                        self.download_type.clone(),
                    ))
                    .width(100.0),
                horizontal_space()
            ]
            .padding(10.0)
        ];
        Container::new(
            column![
                persistent::render_top_buttons(Page::Media),
                buttons_and_labels.align_items(Alignment::Center)
            ]
            .spacing(15),
        )
        .into()
    }
}

// should have option to choose mp4 vs mp3
pub async fn download_content(
    link: String,
    download_path: String,
    mp3_4: String,
) -> Result<String, AppError> {
    // string returned is what the user will see, either an error or download success
    // i dont really see a reason to be able to download playlists like this. who wants to download videos in batch like that? idk idc
    if link.contains("youtube") {
        download_youtube(link, download_path, mp3_4).await?;
    } else if link.contains("instagram") {
        download_insta()?;
    } else {
        return Err(AppError::InvalidUrlError(
            "Link does not contain 'instagram' or 'youtube'".to_string(),
        ));
    }
    todo!()
}

async fn download_youtube(
    link: String,
    mut path: String,
    mp3_4: String,
) -> Result<String, AppError> {
    let settings = if mp3_4 == ".mp3" {
        VideoOptions {
            quality: rusty_ytdl::VideoQuality::HighestAudio,
            filter: rusty_ytdl::VideoSearchOptions::Audio,
            ..Default::default()
        }
    } else {
        VideoOptions {
            quality: rusty_ytdl::VideoQuality::HighestVideo,
            filter: rusty_ytdl::VideoSearchOptions::Video,
            ..Default::default()
        }
    };
    let vid = rusty_ytdl::blocking::Video::new_with_options(link, settings)?;
    // make the path end with a slash
    path = if !path.ends_with('\\') | !path.ends_with('/') {
        format!("{}/", path)
    } else {
        path
    };
    // clean the inputs :D
    let title = crate::yt::interface::clean_inputs_for_win_saving(
        vid.get_basic_info()?.video_details.title,
    );
    let full_output = format!("{}{} - {}{}", path, &title, vid.get_video_id(), mp3_4);
    let new_path = std::path::Path::new(&full_output);
    vid.download(new_path)?;
    Ok(format!("{} downloaded successfully!", title))
}

fn download_insta() -> Result<String, AppError> {
    Err(AppError::SearchError(
        "Instagram downloads are unimplmented".to_string(),
    ))
}
