use crate::gui::messages::{ComboBoxType, ProgramCommands, TextType};
use crate::gui::style::button::punge_button_style;
use crate::types::AppError;
use crate::types::Config;
use iced::widget::{
    button, column, combo_box, row, scrollable, space, text, text_input, Column, Container,
};
use iced::{Alignment, Element, Length};
use itertools::Itertools;
use log::{debug, info, warn};
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
                space().width(Length::Fill),
                text("Youtube / Instagram link").width(175.0),
                text_input(&self.download_input, &self.download_input)
                    .on_input(|txt| {
                        ProgramCommands::UpdateWidgetText(TextType::Mp4DownloadInput, txt)
                    })
                    .width(500.0),
                space().width(Length::Fill),
            ]
            .padding(10.0)
            .align_y(Alignment::Center),
            row![
                space().width(Length::Fill),
                text("Path:").width(175.0),
                text_input(&self.download_to_location, &self.download_to_location)
                    .on_input(|txt| {
                        ProgramCommands::UpdateWidgetText(TextType::Mp4PathInput, txt)
                    })
                    .width(500),
                space().width(Length::Fill)
            ]
            .padding(10.0),
            row![
                space().width(Length::Fill),
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
                space().width(Length::Fill)
            ]
            .padding(10.0)
        ];
        Container::new(
            column![
                buttons_and_labels.align_x(Alignment::Center),
                row![
                    space().width(Length::Fill),
                    scrollable(
                        self.download_feedback
                            .iter()
                            .fold(Column::new(), |item, str| { item.push(text(str)) })
                    ),
                    space().width(Length::Fill)
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
        // mp3 will download jpgs, mp4 selected will download videos
        // maybe this could be more clear...?
        download_insta(link.clone(), download_path, &mp3_4).await?;
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
    let vid = Video::new_with_options(&link, settings)?;
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
    debug!("gonna download video to: {:?}", &new_path);
    crate::yt::cmd::cmd_download_media(&link, &full_output, &vid.get_video_id(), &mp3_4).unwrap();
    debug!("Video download did not throw an error");
    Ok(format!("{} downloaded successfully!", title))
}

async fn download_insta(
    link: String,
    download_dir: String,
    mp3_4: &str,
) -> Result<String, AppError> {
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
    debug!("Started insta download");
    // this needs to block?
    // we also need to check for multiple downloads. They can be a collection of jpgs / mp4
    // we can either check the number of files in the directory (should be n * 4) or do a collection of match checks
    // but how do we tell a thumbnail of a video vs a regular photo? Should we just scan for all jpgs & mp4s and move them all !?
    // should probably do some testing for this...
    match process {
        Ok(_) => {
            info!("download seemed to work fine. downloadir={}", &download_dir);
            let dir_iter = std::fs::read_dir(format!("./-{}/", &split_str[4])).unwrap();
            match mp3_4 {
                ".mp3" => {
                    debug!(".mp3 detected in the user choice");
                    for path in dir_iter {
                        let name = path.unwrap().file_name().into_string().unwrap();
                        debug!("looking for: {}", &name);
                        // i guess for the insance of multiple slides,
                        if name.ends_with(".jpg") {
                            let src_file = format!("./-{}/{}", split_str[4], &name);
                            let f = std::fs::OpenOptions::new()
                                .write(true)
                                .open(&src_file)
                                .unwrap();
                            match f.set_modified(std::time::SystemTime::now()) {
                                Err(e) => {
                                    warn!(
                                        "Unable to reset modified date for downloaded content ({}): {}",
                                        e, &src_file
                                    );
                                }
                                Ok(t) => {
                                    info!("Reset modified date successfully for: {}", &src_file);
                                }
                            }
                            let dst_file = format!("{}/{}", &download_dir, &name);
                            debug!("moving {} to {}", &src_file, &dst_file);
                            match std::fs::copy(src_file, dst_file) {
                                Ok(_) => info!("the jpg copied fine"),
                                Err(e) => {
                                    warn!("failure copying jpg to destination, abortintg: {:?}", e);
                                    return Err(AppError::FileError(
                                        "Failure moving jpg to destiation".to_string(),
                                    ));
                                }
                            }
                        }
                    }
                    debug!("removing directory!!: ./-{}", split_str[4]);
                    std::fs::remove_dir_all(format!("./-{}", split_str[4])).unwrap();
                    return Ok(format!("{} downloaded!!", split_str[4]));
                }
                ".mp4" => {
                    debug!(".mp4 detected in the user choice");
                    for path in dir_iter {
                        let name = path.unwrap().file_name().into_string().unwrap();
                        // i guess for the insance of multiple slides,
                        if name.ends_with(".mp4") {
                            let src_file = format!("./-{}/{}", split_str[4], &name);
                            let f = std::fs::OpenOptions::new()
                                .write(true)
                                .open(&src_file)
                                .unwrap();
                            match f.set_modified(std::time::SystemTime::now()) {
                                Err(e) => {
                                    warn!(
                                        "Unable to reset modified date for downloaded content ({}): {}",
                                        e, &src_file
                                    );
                                }
                                Ok(t) => {
                                    info!("Reset modified date successfully for: {}", &src_file);
                                }
                            }
                            let dst_file = format!("{}/{}", &download_dir, &name);
                            debug!("moving {} to {}", &src_file, &dst_file);
                            match std::fs::copy(&src_file, &dst_file) {
                                Ok(_) => {
                                    info!(
                                        "file copied successfully {} -> {}",
                                        &src_file, &dst_file
                                    );
                                }
                                Err(e) => {
                                    info!("File could not be copied. Likely was not able to be downloaded. {:?}", e);
                                    return Err(AppError::FileError("The file could not be moved from it's temporary location. view logs for more info".to_string()));
                                }
                            };
                        }
                    }
                    debug!("removing directory!!: ./-{}", split_str[4]);
                    match std::fs::remove_dir_all(format!("./-{}", split_str[4])) {
                        Ok(_) => info!("directly removed successfully"),
                        Err(e) => {
                            info!("directory coudld not be removed: {:?}", e);
                            return Err(AppError::FileError(format!(
                                "The directory could not be removed ({}) {:?} ",
                                split_str[4], e,
                            )));
                        }
                    }
                    return Ok(format!("{} downloaded!!", split_str[4]));
                }
                _ => return Err(AppError::FileError("How did we get here?".to_string())),
            }
        }
        Err(e) => {
            return Err(AppError::FileError(format!(
                "instaloader error: {:?}, is it on your path?",
                e
            )))
        }
    }
}
