// this is the rewrite of the youtube stuff.
// we are doing this rewrite for two primary reasons:
// 1. this 'modularity' of having the user's supply their own 'yt-dlp.exe' file makes it so users do not need to
// depend on a new punge release if the downloader stops working (which in turn makes it so I don't have to differenciate between stable & nightly)
// releases, and instead can just have a rolling main branch
// 2. we don't have to depend on rusty-ytdl, because sometimes it is slow to update (as of writing this, it does not download)
// and yt-dlp.exe has like 1000 issues created within 10 seconds if the api changes
use crate::types::AppError;
use log::{debug, info, warn};
use std::process::Command;

pub fn cmd_download(
    link: &str,
    desired_output: &str,
    jpg_full_path: &str,
    id: &str,
) -> Result<String, AppError> {
    // this function is either called directly from `ProgramCommands::Download` or from the playlist_wrapper (hence need for `playlist_title` arg)
    // we need to parse the uniqueid.
    // -o "here.mp3" for output
    // --write-thumbnail
    // -x (converts to audio-only), maybe it is worth to do this ourselves, so we can have some logging output?
    let temp_path = format!("./{}.webm", id); // i think it always gives you a webm...
    info!("downloading to {}", &temp_path);
    // also download the thumbnail!!
    let cmd = Command::new("yt-dlp.exe")
        .args([
            link,
            "-o",
            &temp_path,
            "--write-thumbnail",
            "--convert-thumbnails",
            "jpg",
            // "--output",
            // jpg_full_path,
        ])
        .output();
    match cmd {
        Ok(t) => {
            info!("download successful! {:?} time to convert the file..", &t);
            let ffmpeg_cmd = Command::new("ffmpeg.exe")
                .args([
                    "-i",
                    &temp_path,
                    "-vn",
                    "-c:a",
                    "libmp3lame",
                    "-b:a",
                    "192K",
                    &desired_output,
                ])
                .output();
            match ffmpeg_cmd {
                Ok(t) => {
                    info!(
                        "File converted successfully {:?}!! now lives at: {}",
                        &t, &desired_output
                    );
                    info!("we are going to remove the old file ({})", &temp_path);
                    std::fs::remove_file(&temp_path).unwrap();
                    let old_jpg = format!("./{}.jpg", &id);
                    // since the stupid thumbnail just downloads into the punge directory....
                    debug!("copying {} to {}", &old_jpg, &jpg_full_path);
                    // std::fs::File::create(&old_jpg).unwrap();
                    std::fs::copy(&old_jpg, jpg_full_path).unwrap();
                    // cant fail if the one above doesn't
                    std::fs::remove_file(old_jpg).unwrap();
                    return Ok("Download and convert appears to be successful".to_string());
                }
                Err(e) => {
                    debug!(
                        "the download returned `ok`, ffmpeg operation returned `err`: {:?}",
                        e
                    );
                    return Err(AppError::FfmpegError(
                        "Something went wrong with ffmpeg. this is rare. check the logs"
                            .to_string(),
                    ));
                }
            }
        }
        Err(e) => {
            warn!("download failure: {:?}", &e);
            return Err(AppError::FileError(format!(
                "Something went wrong when downloading: {:?}",
                e,
            )));
        }
    }
}
