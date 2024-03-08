use std::fmt::Formatter;
// types for the 'download' struct. Only used in the gui for sending back data
// in theory, this should allow for multiple downloads at the same time ?
use crate::types::AppError;
use crate::yt::interface::download_interface;
use crate::{gui::messages::ProgramCommands, types::YouTubeData};
use iced::{subscription, Subscription};

pub struct Download {
    pub id: usize,
    pub link: Option<String>,
    pub playlist_title: Option<String>,
    //    link: String // ??? is this right?
}

pub enum DownloadState {
    Ready(String, Option<String>),
    Finished,
}
impl std::fmt::Debug for Download {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.link.clone().unwrap())
    }
}

pub fn subscription_convert(
    id: usize,
    link: String,
    playlist_title: Option<String>,
) -> Subscription<Option<Result<YouTubeData, AppError>>> {
    subscription::unfold(
        id,
        DownloadState::Ready(link, playlist_title),
        move |state| download_int(id, state),
    )
}

pub async fn download_int(
    _id: usize,
    state: DownloadState,
) -> (Option<Result<YouTubeData, AppError>>, DownloadState) {
    // option should hold the state type?? {

    match state {
        DownloadState::Ready(link, playlist_title) => (
            Some(download_interface(link, playlist_title).await),
            DownloadState::Finished,
        ),
        DownloadState::Finished => (None, iced::futures::future::pending().await),
    }
}

impl Download {
    pub fn subscription(&self) -> Subscription<ProgramCommands> {
        match &self.link {
            Some(yt_link) => {
                subscription_convert(self.id, yt_link.clone(), self.playlist_title.clone())
                    .map(ProgramCommands::AddToDownloadFeedback)
            }
            _ => Subscription::none(),
        }
    }
}
