use crate::gui::messages::{Page, ProgramCommands, TextType};
use iced::widget::{button, column, container, row, text, text_input};
use iced::Element;

pub struct SongEditPage {
    pub title: String,
    pub author: String,
    pub album: String,
    pub uniqueid: String,
    pub ischecked: bool,
    pub multi_select: bool,
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
            ischecked: false,
            multi_select: false, // if multiple songs are selected, grey out 'title'
        }
    }
    pub fn update_info(
        &mut self,
        title: String,
        author: String,
        album: String,
        uniqueid: String,
        ischecked: bool,
        multi_select: bool,
    ) {
        self.title = title;
        self.author = author;
        self.album = album;
        self.uniqueid = uniqueid;
        self.ischecked = ischecked;
        self.multi_select = multi_select;
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        let update_or_leave_buttons = row![
            button("Update!").on_press(ProgramCommands::UpdateSong(
                crate::gui::widgets::row::RowData {
                    title: self.title.clone(),
                    author: self.author.clone(),
                    album: self.album.clone(),
                    uniqueid: self.uniqueid.clone(),
                    row_num: 0, // doesn't matter. we're using this type for convinence sake...
                }
            )),
            button(text("Discard")).on_press(ProgramCommands::ChangePage(Page::Main))
        ]
        .spacing(10.0);
        // if multiple songs are selected, we don't want to change the title!!
        let conditional_title = if self.multi_select {
            text_input("", "Multiple selected, title unavailable")
        } else {
            text_input(&self.title, &self.title)
                .on_input(|txt| ProgramCommands::UpdateWidgetText(TextType::TitleChange, txt))
        };
        let text_part = column![
            text("Title"),
            text("Artist"),
            text("Album"),
            update_or_leave_buttons
        ]
        .spacing(30.0);
        let input_part = column![
            conditional_title,
            text_input(&self.author, &self.author)
                .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::AuthorChange, txt) }),
            text_input(&self.album, &self.album)
                .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::AlbumChange, txt) })
        ]
        .spacing(15.0);
        let main_content = row![text_part, input_part];
        column![main_content, container(text("")).height(300)]
            .spacing(50.0)
            .into()
    }
}
