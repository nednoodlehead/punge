use crate::gui::messages::{CheckBoxType, ProgramCommands, TextType};
use crate::gui::style::button::PungeButton;
use crate::types::YouTubeSearchResult;
use iced::widget::{
    button, checkbox, column, horizontal_space, row, scrollable, text, text_input, Column,
    Container,
};
use iced::{Element, Length};

pub struct DownloadPage {
    pub search_text: String,
    pub text: String,
    pub download_feedback: Vec<String>, // feedback to the user to tell them if song was downloaded successfully or not
    pub youtube_content: Vec<YouTubeSearchResult>, // dyncamically created boxes
    pub include_videos: bool,
    pub include_playlists: bool,
}

impl DownloadPage {
    pub fn new() -> Self {
        DownloadPage {
            search_text: "".to_string(),
            text: "".to_string(),
            download_feedback: vec![],
            youtube_content: vec![],
            include_videos: true,
            include_playlists: true,
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        Container::new(
            column![
                row![
                    column![
                        row![
                            text_input("Search YouTube!", &self.search_text).on_input(|txt| {
                                ProgramCommands::UpdateWidgetText(TextType::YouTubeSearchInput, txt)
                            }),
                            button(text("Search!"))
                                .style(iced::theme::Button::Custom(Box::new(PungeButton)))
                                .on_press(ProgramCommands::SearchYouTube(self.search_text.clone()))
                        ],
                        self.create_searcher_scrollable(),
                        row![
                            checkbox("Include Videos", self.include_videos).on_toggle(|val| {
                                ProgramCommands::CheckBoxEvent(CheckBoxType::IncludeVideos, val)
                            }),
                            checkbox("Include Playlists", self.include_playlists).on_toggle(
                                |val| {
                                    ProgramCommands::CheckBoxEvent(
                                        CheckBoxType::IncludePlaylists,
                                        val,
                                    )
                                }
                            )
                        ]
                    ]
                    .spacing(15.0),
                    column![
                        row![
                            text_input("Enter YouTube link here: ", &self.text).on_input(|txt| {
                                ProgramCommands::UpdateWidgetText(TextType::DownloadLinkInput, txt)
                            }),
                            button(text("Download!"))
                                .style(iced::theme::Button::Custom(Box::new(PungeButton)))
                                .on_press(ProgramCommands::Download(self.text.clone()))
                        ],
                        self.create_feedback_scrollable(),
                    ]
                ], // download_row,
                   // feedback_scrollable,
            ]
            .spacing(10.0),
        )
        .into()
    }
    fn create_feedback_scrollable(&self) -> Element<'_, ProgramCommands> {
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

    fn create_searcher_scrollable(&self) -> Element<'_, ProgramCommands> {
        scrollable(
            self.youtube_content
                .iter()
                .fold(Column::new(), |item, results| {
                    let col = match &results.duration {
                        Some(duration) => {
                            // these are normal videos
                            column![row![
                                column![
                                    text(results.title.clone()),
                                    text(results.author.clone()),
                                    text(duration.clone())
                                ]
                                .width(Length::Fixed(400.0)),
                                horizontal_space(),
                                column![
                                    button(text("Download!"))
                                        .style(iced::theme::Button::Custom(Box::new(PungeButton)))
                                        .on_press(ProgramCommands::Download(results.link.clone())),
                                    text("Stream!")
                                ],
                            ]]
                            .padding(10.0)
                        }
                        None => {
                            // these are playlists
                            column![row![
                                column![
                                    text(results.title.clone()),
                                    text(results.author.clone()),
                                    text(results.videos.clone().unwrap())
                                ]
                                .width(Length::Fixed(400.0)),
                                horizontal_space(),
                                column![
                                    button(text("Download!"))
                                        .style(iced::theme::Button::Custom(Box::new(PungeButton)))
                                        .on_press(ProgramCommands::Download(results.link.clone())),
                                    text("Stream!")
                                ],
                            ]]
                            .padding(10.0)
                        }
                    };
                    // push each iteration to the final scrollable
                    item.push(col)
                }),
        )
        .width(Length::Fixed(600.0))
        .height(Length::Fixed(450.0))
        .into()
    }
}
