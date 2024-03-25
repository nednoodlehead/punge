// this file is for elements that are persistent across pages
// such as the 'current playing' bar at the bottom, and the buttons at the top to change pages
use crate::gui::messages::{Page, ProgramCommands};
use iced::widget::{button, text, Row};
use iced::Element;

pub fn render_top_buttons(ignore: Page) -> Element<'static, ProgramCommands> {
    // im not really sure the best way to do this? ig just match based on which to ignore?
    let buttons = [
        ("Main", Page::Main),
        ("Download!", Page::Download),
        ("Media downloader", Page::Media),
        ("Settings", Page::Settings),
        ("Add Playlist", Page::Playlist),
    ];
    let btn = buttons.iter().map(|(txt, page)| {
        if *page == ignore {
            button(text(txt)).into()
        } else {
            button(text(txt))
                .on_press(ProgramCommands::ChangePage(*page))
                .into()
        }
    });
    Row::with_children(btn).spacing(15).into()
}
