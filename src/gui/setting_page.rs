use crate::gui::messages::{Page, ProgramCommands};
use crate::playliststructs::Config;
use iced::widget::{button, column, row, text, text_input, Container};
use iced::Element;

pub struct SettingPage {
    pub config: Config,
    pub backup_text: String,
}

impl SettingPage {
    pub fn new() -> Self {
        let con = Config {
            backup_path: String::from(r"%userprofile%/Documents/"),
        };
        SettingPage {
            config: con.clone(),
            backup_text: con.backup_path.clone(),
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        Container::new(column![
            text("Actual Settings page lol"),
            button(text("Home")).on_press(ProgramCommands::ChangePage(Page::Main)),
            row![
                text_input(&self.backup_text, &self.backup_text)
                    .on_input(ProgramCommands::UpdateBackupText),
                button(text("Backup!")).on_press(ProgramCommands::CreateBackup)
            ],
        ])
        .into()
    }
}
