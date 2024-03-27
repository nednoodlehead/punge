use crate::gui::messages::{Page, ProgramCommands};
use crate::gui::persistent;
use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, Column,
    Container,
};
use iced::{Alignment, Element, Length};

pub struct DownloadPage {
    pub text: String,
    pub download_feedback: Vec<String>, // feedback to the user to tell them if song was downloaded successfully or not
}

impl DownloadPage {
    pub fn new() -> Self {
        DownloadPage {
            text: "".to_string(),
            download_feedback: vec![],
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        let debug_button = button(text("List check")).on_press(ProgramCommands::Debug);
        let input_field = text_input("Paste YouTube links here!", self.text.as_str())
            .on_input(ProgramCommands::UpdateDownloadEntry)
            .width(Length::Fixed(400.0));
        let confirm_button =
            button(text("Download!")).on_press(ProgramCommands::Download(self.text.clone()));
        let download_row = row![
            horizontal_space(),
            input_field,
            confirm_button,
            horizontal_space()
        ]
        .align_items(Alignment::End);
        let feedback_scrollable = row![
            horizontal_space(),
            container(self.create_scrollable()),
            horizontal_space()
        ];
        Container::new(
            column![
                persistent::render_top_buttons(Page::Download),
                download_row,
                feedback_scrollable,
                debug_button
            ]
            .spacing(10.0),
        )
        .into()
    }
    fn create_scrollable(&self) -> Element<'_, ProgramCommands> {
        // not the right output type?
        scrollable(
            self.download_feedback
                .iter()
                .fold(Column::new(), |item, string| item.push(text(string))),
        )
        .height(150.0)
        .width(490.0)
        .into()
    }
}
