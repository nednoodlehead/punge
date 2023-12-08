use iced::widget::{Container, text, button, column, text_input, row, horizontal_space, scrollable, Column, container};
use iced::{Element, Length, Alignment};
use crate::gui::messages::{ProgramCommands, Page};

use crate::gui::scroller;

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
            .on_input(ProgramCommands::UpdateDownloadEntry).width(Length::Fixed(400.0));
        let confirm_button = button(text("Download!")).on_press(ProgramCommands::Download(self.text.clone()));
        let download_row = row![horizontal_space(Length::Fixed(300.0)),input_field, confirm_button].align_items(Alignment::End);
        let feedback_scrollable = row![horizontal_space(Length::Fixed(300.0)), container(self.create_scrollable()).style(iced::theme::Container::Custom(Box::from(scroller::ScrollerContainer)))];
        Container::new(column![text("Download page"), button(text("Home")).on_press(ProgramCommands::ChangePage(Page::Main)), download_row,
        feedback_scrollable, debug_button].spacing(10.0)).into()
    }
    fn create_scrollable(&self) -> Element<'_, ProgramCommands> { // not the right output type?
        scrollable(self.download_feedback.iter().fold(
            Column::new(), |item, string| {
                item.push(text(string))
            }
        )).height(150.0)
            .width(490.0)
            .into()

    }
}
