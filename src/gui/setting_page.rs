use crate::gui::messages::{ComboBoxType, Page, ProgramCommands, TextType};
use crate::gui::persistent;
use crate::types::{Config, PungeKeyBind};
use crate::utils::key::{self};

use iced::widget::{button, column, combo_box, row, text, text_input, Container};
use iced::Element;
use std::hash::Hash;

// all bind options: staticvolup, staticvoldown, forward, bckwards, play/pause, shuffle, gotoalbum
// optional cool binds: add current song to playlist[x]. so main = 0, energy = 1...
// search for song from keyboard input, stop keyboard input via 'enter' event.

// i would love to just have the user press the buttons, but idk how to check focus of widget
// aka, what bind interface are they trying to change (skip, play...)

pub struct SettingPage {
    pub backup_text: String,
    pub mp3_path_text: String,
    pub jpg_path_text: String,
    pub static_increment: String, // the increments are converted into u8 when cache is being wrote
    pub static_reduction: String, // if there is a `counter` type of widget, we can use that, and this can be `u8`
    pub media_path: String,
    pub key_options: combo_box::State<String>,
    pub mod_options: combo_box::State<String>,
    pub play_key_value: String,
    pub play_mod1_value: String,
    pub play_mod2_value: String,
    pub forward_key_value: String,
    pub forward_mod1_value: String,
    pub forward_mod2_value: String,
    pub backward_key_value: String,
    pub backward_mod1_value: String,
    pub backward_mod2_value: String,
    pub shuffle_key_value: String,
    pub shuffle_mod1_value: String,
    pub shuffle_mod2_value: String,
    pub staticup_key_value: String,
    pub staticup_mod1_value: String,
    pub staticup_mod2_value: String,
    pub staticdown_key_value: String,
    pub staticdown_mod1_value: String,
    pub staticdown_mod2_value: String,
    pub gotoalbum_key_value: String,
    pub gotoalbum_mod1_value: String,
    pub gotoalbum_mod2_value: String,
}
// how the hotkey numbers are created !!!
pub fn generate_hash(mods: [String; 2], key: String) -> u32 {
    let mut hotkey_str = String::new();
    if mods.contains(&"SHIFT".to_string()) {
        hotkey_str.push_str("shift+")
    }
    if mods.contains(&"CONTROL".to_string()) {
        hotkey_str.push_str("control+")
    }
    if mods.contains(&"ALT".to_string()) {
        hotkey_str.push_str("alt+")
    }
    if mods.contains(&"SUPER".to_string()) {
        hotkey_str.push_str("super+")
    }
    hotkey_str.push_str(&key.to_string());

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hotkey_str.hash(&mut hasher);
    std::hash::Hasher::finish(&hasher) as u32
}

pub fn strings_to_hashmap(
    key: String,
    mod1: String,
    mod2: String,
    cmd: ProgramCommands,
) -> (u32, PungeKeyBind) {
    let temp_hash = generate_hash([mod1.clone(), mod2.clone()], key.clone());
    let modifer_1 = if mod1.is_empty() {
        None
    } else {
        Some(key::string_to_modifiers(mod1))
    };
    let modifer_2 = if mod2.is_empty() {
        None
    } else {
        Some(key::string_to_modifiers(mod2))
    };
    (
        temp_hash,
        PungeKeyBind {
            code: Some(key::string_to_code(key)), // checked beforehand!
            mod1: modifer_1,
            mod2: modifer_2,
            command: cmd,
        },
    )
}

impl SettingPage {
    pub fn new(config: &Config) -> Self {
        let mut pg = SettingPage {
            backup_text: config.backup_path.clone(),
            mp3_path_text: config.mp3_path.clone(),
            jpg_path_text: config.jpg_path.clone(),
            static_increment: config.static_increment.to_string(),
            static_reduction: config.static_reduction.to_string(),
            media_path: config.media_path.clone(),
            // we tried being funny and having the user dynamically add binds, but it sucks to code
            key_options: combo_box::State::new(key::all_codes()),
            mod_options: combo_box::State::new(key::get_all_modifiers()),
            play_key_value: "".to_string(),
            play_mod1_value: "".to_string(),
            play_mod2_value: "".to_string(),
            forward_key_value: "".to_string(),
            forward_mod1_value: "".to_string(),
            forward_mod2_value: "".to_string(),
            backward_key_value: "".to_string(),
            backward_mod1_value: "".to_string(),
            backward_mod2_value: "".to_string(),
            shuffle_key_value: "".to_string(),
            shuffle_mod1_value: "".to_string(),
            shuffle_mod2_value: "".to_string(),
            staticup_key_value: "".to_string(),
            staticup_mod1_value: "".to_string(),
            staticup_mod2_value: "".to_string(),
            staticdown_key_value: "".to_string(),
            staticdown_mod1_value: "".to_string(),
            staticdown_mod2_value: "".to_string(),
            gotoalbum_key_value: "".to_string(),
            gotoalbum_mod1_value: "".to_string(),
            gotoalbum_mod2_value: "".to_string(),
        };
        for (_, bind) in &config.keybinds {
            match bind.command {
                ProgramCommands::PlayToggle => {
                    pg.play_key_value = bind.code.unwrap().to_string();
                    pg.play_mod1_value = bind.mod1.map_or("".to_string(), key::mod_to_string);
                    pg.play_mod2_value = bind.mod2.map_or("".to_string(), key::mod_to_string);
                }
                ProgramCommands::SkipForwards => {
                    pg.forward_key_value = bind.code.unwrap().to_string();
                    pg.forward_mod1_value = bind.mod1.map_or("".to_string(), key::mod_to_string);
                    pg.forward_mod2_value = bind.mod2.map_or("".to_string(), key::mod_to_string);
                }
                ProgramCommands::SkipBackwards => {
                    pg.backward_key_value = bind.code.unwrap().to_string();
                    pg.backward_mod1_value = bind.mod1.map_or("".to_string(), key::mod_to_string);
                    pg.backward_mod2_value = bind.mod2.map_or("".to_string(), key::mod_to_string);
                }
                ProgramCommands::ShuffleToggle => {
                    pg.shuffle_key_value = bind.code.unwrap().to_string();
                    pg.shuffle_mod1_value = bind.mod1.map_or("".to_string(), key::mod_to_string);
                    pg.shuffle_mod2_value = bind.mod2.map_or("".to_string(), key::mod_to_string);
                }
                ProgramCommands::StaticVolumeUp => {
                    pg.staticup_key_value = bind.code.unwrap().to_string();
                    pg.staticup_mod1_value = bind.mod1.map_or("".to_string(), key::mod_to_string);
                    pg.staticup_mod2_value = bind.mod2.map_or("".to_string(), key::mod_to_string);
                }

                ProgramCommands::StaticVolumeDown => {
                    pg.staticdown_key_value = bind.code.unwrap().to_string();
                    pg.staticdown_mod1_value = bind.mod1.map_or("".to_string(), key::mod_to_string);
                    pg.staticdown_mod2_value = bind.mod2.map_or("".to_string(), key::mod_to_string);
                }
                _ => {}
            }
        }
        pg
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
                text("Static increment bind amount (1 = default): "),
                text_input(&self.static_increment, &self.static_increment).on_input(|txt| {
                    ProgramCommands::UpdateWidgetText(TextType::StaticIncrement, txt)
                })
            ]
            .padding(10.0),
            row![
                text("Static reduction bind amount (1 = default): "),
                text_input(&self.static_reduction, &self.static_reduction).on_input(|txt| {
                    ProgramCommands::UpdateWidgetText(TextType::StaticReduction, txt)
                })
            ]
            .padding(10.0),
            row![
                text("Default Media Download location: "),
                text_input(&self.media_path, &self.media_path).on_input(|txt| {
                    ProgramCommands::UpdateWidgetText(TextType::MediaPath, txt)
                })
            ],
            row![button(text("Save!")).on_press(ProgramCommands::SaveConfig)],
            self.render_keybinds()
        ])
        .into()
    }
}
impl SettingPage {
    fn render_keybinds(&self) -> Element<'_, ProgramCommands> {
        column![
            row![
                text("Play toggle"),
                combo_box(
                    &self.key_options,
                    "Key",
                    Some(&self.play_key_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::PlayKey, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 1",
                    Some(&self.play_mod1_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::PlayModifier1, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 2",
                    Some(&self.play_mod2_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::PlayModifier2, txt) }
                )
            ],
            row![
                text("Skip Forwards"),
                combo_box(
                    &self.key_options,
                    "Key",
                    Some(&self.forward_key_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::ForwardKey, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifer 1",
                    Some(&self.forward_mod1_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::ForwardModifer1, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifer 2",
                    Some(&self.forward_mod2_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::ForwardModifer2, txt) }
                )
            ],
            row![
                text("Skip Backwards"),
                combo_box(
                    &self.key_options,
                    "Key",
                    Some(&self.backward_key_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::BackwardKey, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 1",
                    Some(&self.backward_mod1_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::BackwardModifier1, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 2",
                    Some(&self.backward_mod2_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::BackwardModifier2, txt) }
                )
            ],
            row![
                text("Shuffle toggle"),
                combo_box(
                    &self.key_options,
                    "Key",
                    Some(&self.shuffle_key_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::ShuffleKey, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 1",
                    Some(&self.shuffle_mod1_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::ShuffleModifier1, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 2",
                    Some(&self.shuffle_mod2_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::ShuffleModifier2, txt) }
                ),
            ],
            row![
                text("Static volume up"),
                combo_box(
                    &self.key_options,
                    "Key",
                    Some(&self.staticup_key_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::StaticUpKey, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 1",
                    Some(&self.staticup_mod1_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::StaticUpModifier1, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 2",
                    Some(&self.staticup_mod2_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::StaticUpModifier2, txt) }
                ),
            ],
            row![
                text("Static volume down"),
                combo_box(
                    &self.key_options,
                    "Key",
                    Some(&self.staticdown_key_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::StaticDownKey, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 1",
                    Some(&self.staticdown_mod1_value),
                    |txt| {
                        ProgramCommands::UpdateCombobox(ComboBoxType::StaticDownModifier1, txt)
                    }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 2",
                    Some(&self.staticdown_mod2_value),
                    |txt| {
                        ProgramCommands::UpdateCombobox(ComboBoxType::StaticDownModifier2, txt)
                    }
                ),
            ],
            row![
                text("Go to album"),
                combo_box(
                    &self.key_options,
                    "Key",
                    Some(&self.gotoalbum_key_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::GoToAlbumKey, txt) }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 1",
                    Some(&self.gotoalbum_mod1_value),
                    |txt| {
                        ProgramCommands::UpdateCombobox(ComboBoxType::GoToAlbumModifier1, txt)
                    }
                ),
                combo_box(
                    &self.mod_options,
                    "Modifier 2",
                    Some(&self.gotoalbum_mod2_value),
                    |txt| { ProgramCommands::UpdateCombobox(ComboBoxType::GoToAlbumModifer2, txt) }
                ),
            ]
            .spacing(10)
        ]
        .into()
    }
}
