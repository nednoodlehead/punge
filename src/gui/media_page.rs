use crate::gui::messages::{ComboBoxType, ProgramCommands, TextType};
use crate::gui::style::button::punge_button_style;
use crate::types::AppError;
use crate::types::Config;
use iced::widget::{
    button, column, combo_box, horizontal_space, row, scrollable, text, text_input, Column,
    Container,
};
use iced::{Alignment, Element};
use itertools::Itertools;
use log::debug;
use rusty_ytdl::blocking::Video;
use rusty_ytdl::{self, VideoOptions};

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
                combo_box(&self.mp3_and_4, "", Some(&self.download_type), |txt| {
                    ProgramCommands::UpdateCombobox(ComboBoxType::Mp3Or4, txt)
                })
                .width(50.0),
                button(text("Download!"))
                    .style(|_t, status| punge_button_style(status))
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
                buttons_and_labels.align_items(Alignment::Center),
                row![
                    horizontal_space(),
                    scrollable(
                        self.download_feedback
                            .iter()
                            .fold(Column::new(), |item, str| { item.push(text(str)) })
                    ),
                    horizontal_space()
                ]
                .height(350), // empty space..
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
        download_youtube(link.clone(), download_path, mp3_4).await?;
    } else if link.contains("instagram") {
        // always download mp4...
        download_insta(link.clone(), download_path).await?;
    } else {
        return Err(AppError::InvalidUrlError(
            "Link does not contain 'instagram' or 'youtube'".to_string(),
        ));
    }
    Ok(format!("{} downloaded!", link))
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
            quality: rusty_ytdl::VideoQuality::Highest,
            filter: rusty_ytdl::VideoSearchOptions::VideoAudio,
            ..Default::default()
        }
    };
    let vid = Video::new_with_options(link, settings)?;
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

async fn download_insta(link: String, download_dir: String) -> Result<String, AppError> {
    // we could use the pypi instaloader from cmd to do this ?
    // also, by default, (sort of dumb) it sets the date of the download to the date it was
    // uploaded to insta, so we can rename it near the end to change that..
    // --no-video-thumbnails --no-captions --no-metadata-json
    // links look like: https://www.instagram.com/p/123456789 10 11
    let split_str = link.split('/').collect_vec();
    let process = std::process::Command::new("instaloader")
        .args([
            "--filename-pattern={shortcode}",
            // format!("--dirname-pattern={}", download_dir).as_str(),
            "--",
            format!("-{}", split_str[4]).as_str(),
            "--no-video-thumbnails", // tbh we dont really care about the rest..
            "--no-captions",
            "--no-metadata-json",
            "--no-metadata-txt",
            "--no-compress-json",
        ])
        .spawn()
        .unwrap()
        .wait_with_output();
    // this needs to block?
    match process {
        Ok(_) => {
            // so the filename to move can either be `.jpg` or `.mp4`
            let jpg_filename = format!("{}.mp4", split_str[4]);
            let mp4_filename = format!("{}.jpg", split_str[4]);
            // determine which version we have
            let author_file = if std::path::Path::new(&mp4_filename).exists() {
                mp4_filename
            } else {
                jpg_filename
            };
            let src_file = format!("./-{}/{}", split_str[4], author_file);
            let dst_file = format!("{}/{}", download_dir, author_file);
            // put it in the correct location
            debug!(
                "moving {} ({}) to {} ({})",
                &src_file,
                std::path::Path::new(&src_file).exists(),
                &dst_file,
                std::path::Path::new(&dst_file).exists()
            );
            std::fs::copy(src_file, dst_file).unwrap();
            // remove the old directory
            std::fs::remove_dir_all(format!("./-{}", split_str[4])).unwrap();

            // std::fs::File::set_times(, )
            // 2. move the mp4 to the correct directory
            // yadda
            Ok(link)
        }
        Err(e) => {
            Err(AppError::FileError(format!(
                "instaloader error: {:?}. is it on your path!?",
                e
            )))
        }
    }
}
