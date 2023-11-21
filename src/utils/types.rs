use core::fmt;
use std::fmt::Formatter;
// types for the 'download' struct. Only used in the gui for sending back data
// in theory, this should allow for multiple downloads at the same time ?
use crate::gui::messages::ProgramCommands;
use crate::playliststructs::AppError;
use crate::utils::youtube_interface;
use iced::{subscription, Subscription};

pub struct Download {
    pub id: usize,
    pub link: Option<String>,
    //    link: String // ??? is this right?
}

pub enum DownloadState {
    Ready(String),
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
) -> Subscription<Option<Vec<Result<(String, String), AppError>>>> {
    subscription::unfold(id, DownloadState::Ready(link), move |state| {
        download_int(id, state)
    })
}

pub async fn download_int(
    id: usize,
    state: DownloadState,
) -> (
    Option<Vec<Result<(String, String), AppError>>>,
    DownloadState,
) {
    // option should hold the state type?? {

    match state {
        DownloadState::Ready(link) => (
            Some(youtube_interface::download(link).await),
            DownloadState::Finished,
        ),
        DownloadState::Finished => (None, iced::futures::future::pending().await),
    }
}

impl Download {
    pub fn subscription(&self) -> Subscription<ProgramCommands> {
        match &self.link {
            Some(yt_link) => subscription_convert(self.id, yt_link.clone())
                .map(ProgramCommands::AddToDownloadFeedback),
            _ => Subscription::none(),
        }
    }
}
