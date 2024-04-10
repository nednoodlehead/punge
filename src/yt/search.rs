use crate::gui::messages::ProgramCommands;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element};
use rusty_ytdl::blocking::search::YouTube;

pub async fn see_content(search: String) -> Vec<rusty_ytdl::search::SearchResult> {
    let yt = YouTube::new().unwrap();
    let options = rusty_ytdl::search::SearchOptions {
        search_type: rusty_ytdl::search::SearchType::All,
        limit: 25,
        safe_search: true,
    };
    yt.search(search, Some(&options)).unwrap()
}
