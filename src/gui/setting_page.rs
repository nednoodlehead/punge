use iced::widget::{Container, text, button, column, vertical_space, text_input};
use iced::{Element};
use crate::gui::messages::{PungeCommand, ProgramCommands, Page};
use crate::gui::start::App;

pub struct SettingPage;

impl SettingPage {
    pub fn new() -> Self {
        SettingPage
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        Container::new(column![text("Actual Settings page lol"), button(text("Home")).on_press(ProgramCommands::ChangePage(Page::Main))]).into()
    }
}
