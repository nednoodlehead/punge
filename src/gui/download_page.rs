use crate::gui::messages::{Page, ProgramCommands};
use crate::gui::persistent;
use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input, Column,
    Container, Row,
};
use iced::{Alignment, Element, Length};

pub struct DownloadPage {
    pub text: String,
    pub download_feedback: Vec<String>, // feedback to the user to tell them if song was downloaded successfully or not
    pub youtube_content: Vec<rusty_ytdl::search::SearchResult>, // dyncamically created boxes
}

impl DownloadPage {
    pub fn new() -> Self {
        DownloadPage {
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
                    self.create_feedback_scrollable(),
                    self.create_searcher_scrollable()
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
                    // custom type ?
                    let col = match results {
                        rusty_ytdl::blocking::search::SearchResult::Video(vid) => column![
                            text(vid.title.clone()),
                            text(vid.channel.name.clone()),
                            text(vid.duration_raw.clone()),
                            // text(vid.uploaded_at.clone().unwrap())
                        ],
                        rusty_ytdl::blocking::search::SearchResult::Playlist(playlist) => {
                            // ok this actually works...?
                            let play = rusty_ytdl::blocking::search::Playlist::get(playlist.url.clone(), None).unwrap();
                            column![
                                text(playlist.name.clone()),
                                text(playlist.channel.name.clone()),
                                text(format!("{} Videos", play.videos.len())),
                            ]
                        }
                        rusty_ytdl::blocking::search::SearchResult::Channel(_chn) => {
                            // can we ignore this !?
                            column![]
                        }
                    };
                    item.push(col)
                }),
        )
        .into()
    }
}
