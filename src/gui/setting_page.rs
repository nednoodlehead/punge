use iced::widget::{Container, text, button, column};
use iced::{Element};
use crate::gui::messages::{ProgramCommands, Page};


pub struct SettingPage;

impl SettingPage {
    pub fn new() -> Self {
        SettingPage
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        Container::new(column![text("Actual Settings page lol"), button(text("Home")).on_press(ProgramCommands::ChangePage(Page::Main))]).into()
    }
}
