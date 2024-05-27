// this file is for elements that are persistent across pages
// such as the 'current playing' bar at the bottom, and the buttons at the top to change pages
use crate::gui::messages::{Page, ProgramCommands};
use crate::gui::start::App;
use crate::gui::style::button::{JustText, MenuButton};
use crate::gui::style::container::{BottomBarContainer, ContainerWithBorder};
use crate::gui::style::scrubber::ScrubberStyle;
use crate::gui::style::volume::VolumeStyle;
use iced::widget::{button, column, container, horizontal_space, row, slider, text, Column, Row};
use iced::{Alignment, Element, Length};
use iced_aw::menu::{Item, Menu};
use iced_aw::widgets::quad;
use iced_aw::widgets::InnerBounds;
use iced_core::{Border, Color};

impl App {
    pub fn render_bottom_bar(&self) -> Element<'static, ProgramCommands> {
        let curr_song = self.current_song.load();
        let search_container = container(row![
            iced::widget::text_input("GoTo closest match", self.search.as_str())
                .on_input(ProgramCommands::UpdateSearch)
                .width(Length::Fixed(250.0)),
            button(text("Confirm")).on_press(ProgramCommands::GoToSong)
        ]);
        container(
            row![
                column![
                    text(curr_song.title.clone()),
                    text(curr_song.author.clone()),
                    text(curr_song.album.clone()),
                ]
                .width(200.0),
                horizontal_space(),
                column![
                    // music buttons & srubbing bar
                    row![
                        horizontal_space(),
                        button(text("<--")).on_press(ProgramCommands::SkipBackwards),
                        button(text(if self.is_paused { "Play" } else { "Stop" }))
                            .on_press(ProgramCommands::PlayToggle),
                        button(text("-->")).on_press(ProgramCommands::SkipForwards),
                        horizontal_space()
                    ]
                    .align_items(Alignment::Center)
                    .spacing(50),
                    row![
                        text(crate::utils::time::sec_to_time(self.time_elapsed)),
                        slider(
                            0..=self.total_time * 10,
                            self.scrubber,
                            ProgramCommands::MoveSlider
                        )
                        // .width(300.0)
                        .style(iced::theme::Slider::Custom(Box::new(ScrubberStyle))),
                        text(crate::utils::time::sec_to_time(self.total_time))
                    ]
                    .spacing(25)
                ]
                .width(450),
                horizontal_space(),
                column![
                    // shuffle, vol & goto
                    row![
                        // shuffle and volume
                        button(text(format!(
                            "Shuffle ({})",
                            if self.shuffle { "On" } else { "Off" }
                        )))
                        .on_press(ProgramCommands::ShuffleToggle),
                        slider(0..=30, self.volume, ProgramCommands::VolumeChange)
                            .width(150)
                            .style(iced::theme::Slider::Custom(Box::new(VolumeStyle)))
                    ]
                    .spacing(15),
                    search_container
                ]
            ]
            .padding(15)
            // .spacing(400)
            .align_items(Alignment::Center),
        )
        .style(iced::theme::Container::Custom(Box::new(BottomBarContainer)))
        .into()
    }
    pub fn render_buttons_side(&self, ignore: Page) -> Element<'static, ProgramCommands> {
        let playlist_add_to_menu = Item::with_menu(
            text("Add to:"),
            Menu::new(
                self.user_playlists
                    .iter()
                    .map(|p| {
                        Item::new(
                            button(text(p.title.clone()))
                                .on_press(ProgramCommands::AddToPlaylist(p.uniqueid.clone())),
                        )
                    })
                    .collect(),
            )
            .max_width(150.0)
            .offset(10.0),
        );
        let menu = iced_aw::menu_bar!((
            button("Edit song")
                .style(iced::theme::Button::Custom(Box::new(MenuButton)))
                .on_press(ProgramCommands::Debug),
            Menu::new(vec![
                Item::new(button(text("Full Edit")).on_press(ProgramCommands::OpenSongEditPage)),
                Item::new(
                    button(text("Swap Title & Author"))
                        .on_press(ProgramCommands::QuickSwapTitleAuthor),
                ),
                Item::new(button(text("Delete!!")).on_press(ProgramCommands::DeleteSong)),
                playlist_add_to_menu,
            ])
            .max_width(180.0)
        ));

        let mut all_playlists_but_main = self.user_playlists.clone();
        // user should always have the 'main' playlist

        all_playlists_but_main.remove(0);
        let playlist_buttons: Vec<Element<ProgramCommands>> = self
            .user_playlists
            .iter()
            .map(|playlist| {
                button(text(playlist.title.clone()))
                    .on_press(ProgramCommands::ChangeViewingPlaylist(
                        playlist.uniqueid.clone(),
                    ))
                    .style(iced::theme::Button::Custom(Box::new(JustText)))
                    .height(Length::Fixed(32.5)) // playlist button height :)
                    .into()
            })
            .collect();
        let buttons = [
            ("Home", Page::Main),
            ("Download!", Page::Download),
            ("Media downloader", Page::Media),
            ("Settings", Page::Settings),
            ("Add Playlist", Page::Playlist),
        ];
        let mut btn: Vec<Element<ProgramCommands>> = buttons
            .iter()
            .map(|(txt, page)| {
                if *page == ignore {
                    button(text(txt))
                        .style(iced::theme::Button::Custom(Box::new(MenuButton)))
                        .into()
                } else {
                    button(text(txt))
                        .style(iced::theme::Button::Custom(Box::new(MenuButton)))
                        .on_press(ProgramCommands::ChangePage(*page))
                        .into()
                }
            })
            .collect();
        btn.push(menu.into());
        btn.push(self.separator().into()); // separater between buttons and playlists :)
        btn.extend(playlist_buttons);
        container(Column::with_children(btn).spacing(5))
            .height(Length::Fill)
            .style(iced::theme::Container::Custom(Box::new(
                ContainerWithBorder,
            )))
            .into()
    }
    // pub fn render_search_result_box(
    //     &self,
    //     title: String,
    //     channel_name: String,
    //     views: String,
    //     duration: String,
    //     link: String,
    //     thumbnail: String,
    // ) -> Element<'_, ProgramCommands> {
    //     // create a container that holds all of the stuff relating to a download
    //     // also downloads the images. they should be flushed on each search
    //     container()
    // }
    fn separator(&self) -> quad::Quad {
        let mut quader = quad::Quad {
            quad_color: Color::from([0.5; 3]).into(),
            quad_border: Border {
                radius: [3.0; 4].into(),
                ..Default::default()
            },
            inner_bounds: InnerBounds::Ratio(0.98, 0.2),
            height: Length::Fixed(20.0),
            ..Default::default()
        };
        // is this like the only way to set it ..?
        quader.width = Length::Fixed(150.0);
        quader
    }
}
