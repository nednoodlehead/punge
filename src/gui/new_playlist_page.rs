use crate::gui::messages::{ProgramCommands, TextType};
use crate::gui::style::button::punge_button_style;
use iced::widget::{button, column, container, horizontal_space, row, text, text_input};
use iced::Element;

// maybe have ability to update playlists from here?
pub struct PlaylistPage {
    pub user_title: String,
    pub user_description: String,
    pub user_thumbnail: String,
    pub user_id: Option<String>, // if some, updating playlist, if none, new playlist :)
}

impl PlaylistPage {
    pub fn new(user_id: Option<String>) -> Self {
        PlaylistPage {
            user_title: "".to_string(),
            user_description: "".to_string(),
            user_thumbnail: "".to_string(),
            user_id,
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        let labels = column![
            text("Title: "),
            text("Description: "),
            text("Path to thumbnail: ")
        ]
        .spacing(20)
        .padding(10);
        let fields = column![
            text_input(&self.user_title, &self.user_title)
                .on_input(|txt| { ProgramCommands::UpdateWidgetText(TextType::UserTitle, txt) }),
            text_input(&self.user_description, &self.user_description).on_input(|txt| {
                ProgramCommands::UpdateWidgetText(TextType::UserDescription, txt)
            }),
            text_input(&self.user_thumbnail, &self.user_thumbnail).on_input(|txt| {
                ProgramCommands::UpdateWidgetText(TextType::UserThumbnail, txt)
            })
        ]
        .spacing(10)
        .padding(10);
        let rows_and_labels = row![labels, fields];
        container::Container::new(column![
            rows_and_labels,
            if self.user_id.is_some() {
                // variable button
                container(row![
                    column![
                        button(text(format!("Update {}", &self.user_title)))
                            .style(|_t, status| punge_button_style(status))
                            .on_press(ProgramCommands::UpdatePlaylist),
                        button(text("Stop editing"))
                            .on_press(ProgramCommands::ClearPlaylistPage)
                            .style(|_t, status| punge_button_style(status))
                    ],
                    horizontal_space(),
                    button(text("Delete playlist (no undo)"))
                        .on_press(ProgramCommands::DeletePlaylist(
                            self.user_id.clone().unwrap()
                        ))
                        .style(|_, status| punge_button_style(status))
                ])
            } else {
                container(
                    button(text("Create!"))
                        .on_press(ProgramCommands::NewPlaylist)
                        .style(|_t, status| punge_button_style(status)),
                )
            },
            container(text("")).height(360)
        ])
        .into()
    }
}
