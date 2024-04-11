use crate::gui::messages::{Page, ProgramCommands, TextType};
use crate::gui::persistent;
use crate::types::YouTubeSearchResult;
use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, Column,
    Container, Row,
};
use iced::{Alignment, Element, Length};

pub struct DownloadPage {
    pub search_text: String,
    pub text: String,
    pub download_feedback: Vec<String>, // feedback to the user to tell them if song was downloaded successfully or not
    pub youtube_content: Vec<YouTubeSearchResult>, // dyncamically created boxes
}

impl DownloadPage {
    pub fn new() -> Self {
        DownloadPage {
            search_text: "".to_string(),
            text: "".to_string(),
            download_feedback: vec![],
            youtube_content: vec![],
        }
    }
    pub fn view(&self) -> Element<'_, ProgramCommands> {
        // let feedback_and_search = row![container(self.create_scrollable()), scrollable];
        let search_test =
            button(text("YOUTUBE")).on_press(ProgramCommands::SearchYouTube("".to_string()));
        // let input_field = text_input("Paste YouTube links here!", self.text.as_str())
        //     .on_input(ProgramCommands::UpdateDownloadEntry)
        //     .width(Length::Fixed(400.0));
        // let confirm_button =
        //     button(text("Download!")).on_press(ProgramCommands::Download(self.text.clone()));
        // let download_row = row![
        //     horizontal_space(),
        //     input_field,
        //     confirm_button,
        //     horizontal_space()
        // ]
        // .align_items(Alignment::End);
        // let feedback_scrollable = row![
        //     horizontal_space(),
        //     container(self.create_scrollable()),
        //     horizontal_space()
        // ];
        Container::new(
            column![
                persistent::render_top_buttons(Page::Download),
                row![
                    column![
                        row![
                            text_input(&self.search_text, &self.search_text).on_input(|txt| {
                                ProgramCommands::UpdateWidgetText(TextType::YouTubeSearchInput, txt)
                            }),
                            button(text("Search!"))
                                .on_press(ProgramCommands::SearchYouTube(self.search_text.clone()))
                        ],
                        self.create_searcher_scrollable(),
                    ]
                    .spacing(15.0),
                    column![
                        row![
                            text_input(&self.text, &self.text).on_input(|txt| {
                                ProgramCommands::UpdateWidgetText(TextType::DownloadLinkInput, txt)
                            }),
                            button(text("Download!"))
                                .on_press(ProgramCommands::Download(self.text.clone()))
                        ],
                        self.create_feedback_scrollable(),
                    ],
                ], // download_row,
                // feedback_scrollable,
                search_test,
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
                                .width(Length::Fixed(500.0)),
                                horizontal_space(),
                                column![
                                    button(text("Download!"))
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
                                .width(500.0),
                                horizontal_space(),
                                column![
                                    button(text("Download!"))
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
        .height(Length::Fixed(500.0))
        .into()
    }
}
