use crate::gui::messages::{Page, ProgramCommands, TextType};
use crate::gui::persistent;
use crate::types::Config;
use crate::utils::cache;
use iced::widget::{button, column, row, text, text_input, Container};
use iced::Element;

pub struct SettingPage {
    pub backup_text: String,
    pub mp3_path_text: String,
    pub jpg_path_text: String,
    pub static_increment: String, // the increments are converted into u8 when cache is being wrote
    pub static_reduction: String, // if there is a `counter` type of widget, we can use that, and this can be `u8`
}

impl SettingPage {
    pub fn new() -> Self {
        let config_obj = match cache::read_from_cache() {
            Ok(t) => t,
            Err(e) => {
                println!("error gettin cache {:?}", e);
                Config {
                    backup_path: format!("C:/Users/{}/Documents/", whoami::username()),
                    mp3_path: String::from("C:/"),
                    jpg_path: String::from("C:/"),
                    static_increment: 1,
                    static_reduction: 1,
                }
            }
        };
        SettingPage {
            backup_text: config_obj.backup_path.clone(),
            mp3_path_text: config_obj.mp3_path.clone(),
            jpg_path_text: config_obj.jpg_path.clone(),
            static_increment: config_obj.static_increment.to_string(),
            static_reduction: config_obj.static_reduction.to_string(),
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        Container::new(column![
            persistent::render_top_buttons(Page::Settings),
            row![
                text("Backup location directory: "),
                text_input(&self.backup_text, &self.backup_text).on_input(|txt| {
                    ProgramCommands::UpdateWidgetText(TextType::BackupText, txt)
                }),
                button(text("Backup!")).on_press(ProgramCommands::CreateBackup)
            ]
            .padding(10.0),
            row![
                text("Mp3 download location"),
                text_input(&self.mp3_path_text, &self.mp3_path_text)
                    .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::Mp3Text, txt) }),
            ]
            .padding(10.0),
            row![
                text("Jpg download location"),
                text_input(&self.jpg_path_text, &self.jpg_path_text)
                    .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::JpgText, txt) })
            ]
            .padding(10.0),
            row![
                text("Static increment bind amount 0.005 = default: "),
                text_input(&self.static_increment, &self.static_increment).on_input(|txt| {
                    ProgramCommands::UpdateWidgetText(TextType::StaticIncrement, txt)
                })
            ]
            .padding(10.0),
            row![
                text("Static reduction bind amount 0.005 = default: "),
                text_input(&self.static_reduction, &self.static_reduction).on_input(|txt| {
                    ProgramCommands::UpdateWidgetText(TextType::StaticReduction, txt)
                })
            ]
            .padding(10.0),
            row![button(text("Save!")).on_press(ProgramCommands::SaveConfig)]
        ])
        .into()
    }
}
