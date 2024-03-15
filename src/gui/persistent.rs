// this file is for elements that are persistent across pages
// such as the 'current playing' bar at the bottom, and the buttons at the top to change pages
use crate::gui::messages::{Page, ProgramCommands};
use iced::widget::{button, row, slider, text, text_input};
use iced::Element;

pub fn render_top_buttons(ignore: Page) -> Element<'static, ProgramCommands> {
    // im not really sure the best way to do this? ig just match based on which to ignore?
    match ignore {
        Page::Main => row![
            button(text("Main")),
            button(text("Download!")).on_press(ProgramCommands::ChangePage(Page::Download)),
            button(text("Mp4 & Insta Downloader"))
                .on_press(ProgramCommands::ChangePage(Page::Media)),
            button(text("Settings")).on_press(ProgramCommands::ChangePage(Page::Settings)),
            button(text("Add Playlist")).on_press(ProgramCommands::ChangePage(Page::Playlist)),
        ]
        .spacing(15)
        .into(),
        Page::Settings => row![
            button(text("Main")).on_press(ProgramCommands::ChangePage(Page::Main)),
            button(text("Download!")).on_press(ProgramCommands::ChangePage(Page::Download)),
            button(text("Mp4 & Insta Downloader"))
                .on_press(ProgramCommands::ChangePage(Page::Media)),
            button(text("Settings")),
            button(text("Add Playlist")).on_press(ProgramCommands::ChangePage(Page::Playlist)),
        ]
        .spacing(15)
        .into(),
        Page::Download => row![
            button(text("Main")).on_press(ProgramCommands::ChangePage(Page::Main)),
            button(text("Download!")),
            button(text("Mp4 & Insta Downloader"))
                .on_press(ProgramCommands::ChangePage(Page::Media)),
            button(text("Settings")).on_press(ProgramCommands::ChangePage(Page::Settings)),
            button(text("Add Playlist")).on_press(ProgramCommands::ChangePage(Page::Playlist)),
        ]
        .spacing(15)
        .into(),
        Page::Playlist => row![
            button(text("Main")).on_press(ProgramCommands::ChangePage(Page::Main)),
            button(text("Download!")).on_press(ProgramCommands::ChangePage(Page::Download)),
            button(text("Mp4 & Insta Downloader"))
                .on_press(ProgramCommands::ChangePage(Page::Media)),
            button(text("Settings")).on_press(ProgramCommands::ChangePage(Page::Settings)),
            button(text("Add Playlist")),
        ]
        .spacing(15)
        .into(),
        Page::Media => row![
            button(text("Main")).on_press(ProgramCommands::ChangePage(Page::Main)),
            button(text("Download!")).on_press(ProgramCommands::ChangePage(Page::Download)),
            button(text("Mp4 & Insta Downloader")),
            button(text("Settings")).on_press(ProgramCommands::ChangePage(Page::Settings)),
            button(text("Add Playlist")).on_press(ProgramCommands::ChangePage(Page::Playlist)),
        ]
        .spacing(15)
        .into(),
    }
}
