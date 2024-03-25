use crate::gui::messages::{Page, ProgramCommands, TextType};
use crate::gui::persistent::render_top_buttons;
use iced::widget::{button, column, container, row, text, text_input};
use iced::Element;

pub struct SongEditPage {
    pub title: String,
    pub author: String,
    pub album: String,
    pub uniqueid: String,
    // features !?
    // also count column, once that happens ...
}

impl SongEditPage {
    pub fn new() -> Self {
        SongEditPage {
            title: "".to_string(),
            author: "".to_string(),
            album: "".to_string(),
            uniqueid: "".to_string(),
        }
    }
    pub fn update_info(&mut self, title: String, author: String, album: String, uniqueid: String) {
        self.title = title;
        self.author = author;
        self.album = album;
        self.uniqueid = uniqueid;
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        let update_or_leave_buttons = row![
            button("Update!").on_press(ProgramCommands::UpdateSong(crate::gui::table::Row {
                title: self.title.clone(),
                author: self.author.clone(),
                album: self.album.clone(),
                uniqueid: self.uniqueid.clone()
            })),
            button(text("Discard")).on_press(ProgramCommands::ChangePage(Page::Main))
        ]
        .spacing(10.0);
        let text_part = column![
            text("Title"),
            text("Artist"),
            text("Album"),
            update_or_leave_buttons
        ]
        .spacing(30.0);
        let input_part = column![
            text_input(&self.title, &self.title)
                .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::TitleChange, txt) }),
            text_input(&self.author, &self.author)
                .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::AuthorChange, txt) }),
            text_input(&self.album, &self.album)
                .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::AlbumChange, txt) })
        ]
        .spacing(15.0);
        let main_content = row![text_part, input_part];
        column![render_top_buttons(Page::SongEdit), main_content]
            .spacing(50.0)
            .into()
    }
}
