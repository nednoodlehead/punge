// this file is for elements that are persistent across pages
// such as the 'current playing' bar at the bottom, and the buttons at the top to change pages
use crate::gui::messages::{Page, ProgramCommands};
use crate::gui::start::App;
use iced::widget::{
    button, column, container, horizontal_space, row, slider, text, vertical_space, Row,
};
use iced::{Alignment, Element, Length};

pub fn render_top_buttons(ignore: Page) -> Element<'static, ProgramCommands> {
    // im not really sure the best way to do this? ig just match based on which to ignore?
    let buttons = [
        ("Main", Page::Main),
        ("Download!", Page::Download),
        ("Media downloader", Page::Media),
        ("Settings", Page::Settings),
        ("Add Playlist", Page::Playlist),
    ];
    let btn = buttons.iter().map(|(txt, page)| {
        if *page == ignore {
            button(text(txt)).into()
        } else {
            button(text(txt))
                .on_press(ProgramCommands::ChangePage(*page))
                .into()
        }
    });
    Row::with_children(btn).spacing(15).into()
}

impl App {
    pub fn render_bottom_bar(&self) -> Element<'static, ProgramCommands> {
        let curr_song = self.current_song.load();
        let search_container = container(row![
            iced::widget::text_input("GoTo closest match", self.search.as_str())
                .on_input(ProgramCommands::UpdateSearch)
                .width(Length::Fixed(250.0)),
            button(text("Confirm")).on_press(ProgramCommands::GoToSong)
        ]);
        container(column![
            row![
                column![
                    text(curr_song.title.clone()),
                    text(curr_song.author.clone()),
                    text(curr_song.album.clone())
                ]
                .padding(2.5)
                .width(225.0),
                button(text("Go back")).on_press(ProgramCommands::SkipBackwards),
                button(text(if self.is_paused { "Play!" } else { "Stop!" }))
                    .on_press(ProgramCommands::PlayToggle),
                button(text("Go forwards")).on_press(ProgramCommands::SkipForwards),
                button(text(format!(
                    "Shuffle ({})",
                    if self.shuffle { "On" } else { "Off" }
                )))
                .on_press(ProgramCommands::ShuffleToggle),
                column![
                    slider(0..=30, self.volume, ProgramCommands::VolumeChange).width(150),
                    search_container
                ]
                .align_items(Alignment::Center)
                .spacing(5.0)
            ]
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .spacing(50.0),
            row![
                horizontal_space(),
                text(self.time_elapsed),
                // slider also needs to have a dynamic range. 1 step should equal 1 second
                slider(
                    0..=self.total_time,
                    self.scrubber,
                    ProgramCommands::MoveSlider
                )
                .on_release(ProgramCommands::SkipToSeconds(self.scrubber)),
                text(crate::utils::time::sec_to_time(self.total_time)), // todo conver to
                horizontal_space()
            ]
            .spacing(10.0)
        ])
        .into()
    }
}
