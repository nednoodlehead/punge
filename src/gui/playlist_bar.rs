// this file contains the GUI stuff for the right-side bar that contains all the user's playlists
// no images are shown in this pane. Not sure if I want that to change or not.

use crate::db::fetch::get_all_playlists;
use crate::gui::messages::ProgramCommands;
use crate::playliststructs::UserPlaylist;
use iced::widget::{button, text, Text};
use iced::widget::{Column, Scrollable};
use iced::Element;

use crate::gui::start::App;

impl App {
    pub fn render_sidebar(&self) -> Element<'_, ProgramCommands> {
        let playlists: Vec<UserPlaylist> = get_all_playlists().unwrap();
        let to_text = playlists.iter().map(|p| text(p)).collect::<Vec<Text>>();
        let scroller = Scrollable::new(playlists.iter().fold(
            Column::new().spacing(10.0),
            |item, string| {
                item.push(
                    button(text(string.clone()))
                        .on_press(ProgramCommands::ChangeViewingPlaylist(string.to_owned())),
                )
            },
        ))
        .height(500.0)
        .width(150.0);
        scroller.into()
    }
}
