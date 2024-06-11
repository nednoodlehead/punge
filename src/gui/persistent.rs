// this file is for elements that are persistent across pages
// such as the 'current playing' bar at the bottom, and the buttons at the top to change pages
use crate::gui::messages::{Page, ProgramCommands};
use crate::gui::start::App;
use crate::gui::style::button::{JustText, MenuButton, PlaylistText, SubMenuButton};
use crate::gui::style::container::BottomBarContainer;
use crate::gui::style::menu::PungeMenu;
use crate::gui::style::scrubber::ScrubberStyle;
use crate::gui::style::volume::VolumeStyle;
use iced::widget::{button, column, container, horizontal_space, row, slider, text, Column, Image};
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
                .width(Length::Fixed(200.0)),
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
                        button(Image::new("./img/punge_left_new.png"))
                            .style(iced::theme::Button::Custom(Box::new(JustText)))
                            .on_press(ProgramCommands::SkipBackwards),
                        button(if self.is_paused {
                            Image::new("./img/punge_play_new.png")
                        } else {
                            Image::new("./img/punge_pause_new.png")
                        })
                        .style(iced::theme::Button::Custom(Box::new(JustText)))
                        .height(50)
                        .width(50)
                        .on_press(ProgramCommands::PlayToggle),
                        button(Image::new("./img/punge_right_new.png"))
                            .on_press(ProgramCommands::SkipForwards)
                            .style(iced::theme::Button::Custom(Box::new(JustText))),
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
                        .on_release(ProgramCommands::SkipToSeconds(self.scrubber / 10))
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
                        button(if self.shuffle {
                            Image::new("./img/shuffle_on_new.png")
                        } else {
                            Image::new("./img/shuffle_off_new.png")
                        })
                        .height(50)
                        .width(50)
                        .style(iced::theme::Button::Custom(Box::new(JustText)))
                        .on_press(ProgramCommands::ShuffleToggle),
                        slider(0..=30, self.volume, ProgramCommands::VolumeChange)
                            .width(150)
                            .style(iced::theme::Slider::Custom(Box::new(VolumeStyle)))
                    ]
                    .align_items(Alignment::Center)
                    .spacing(15),
                    search_container
                ]
                .spacing(5)
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
            button(text("Add to...                     ->")) // this is mad goofy lol
                .width(180)
                .style(iced::theme::Button::Custom(Box::new(SubMenuButton)))
                .on_press(ProgramCommands::Debug),
            Menu::new(
                self.user_playlists
                    .iter()
                    .map(|p| {
                        Item::new(
                            button(text(p.title.clone()))
                                .style(iced::theme::Button::Custom(Box::new(SubMenuButton)))
                                .width(150)
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
                Item::new(
                    button(text("Full Edit"))
                        .on_press(ProgramCommands::OpenSongEditPage)
                        .style(iced::theme::Button::Custom(Box::new(SubMenuButton)))
                        .width(180)
                ),
                Item::new(
                    button(text("Swap Title & Author"))
                        .style(iced::theme::Button::Custom(Box::new(SubMenuButton)))
                        .on_press(ProgramCommands::QuickSwapTitleAuthor)
                        .width(180),
                ),
                Item::new(
                    button(text("Delete!!"))
                        .style(iced::theme::Button::Custom(Box::new(SubMenuButton)))
                        .on_press(ProgramCommands::DeleteSong)
                        .width(180)
                ),
                playlist_add_to_menu,
            ])
            .offset(0.0)
            .max_width(180.0)
        ))
        .style(iced_aw::style::MenuBarStyle::Custom(Box::new(PungeMenu)));

        let mut all_playlists_but_main = self.user_playlists.clone();
        // user should always have the 'main' playlist

        all_playlists_but_main.remove(0);
        let playlist_buttons: Vec<Element<ProgramCommands>> = self
            .user_playlists
            .iter()
            .map(|playlist| {
                let dropdown = iced_aw::additional_menu::Item::with_menu(
                    button(text(playlist.title.clone()))
                        .style(iced::theme::Button::Custom(Box::new(PlaylistText))),
                    iced_aw::additional_menu::Menu::new(vec![
                        iced_aw::additional_menu::Item::new(
                            button(text("Edit"))
                                .on_press(ProgramCommands::OpenPlaylistEditPage(playlist.clone()))
                                .width(180)
                                .style(iced::theme::Button::Custom(Box::new(SubMenuButton))),
                        ),
                        iced_aw::additional_menu::Item::new(
                            button(text("Duplicate?"))
                                .width(180)
                                .style(iced::theme::Button::Custom(Box::new(SubMenuButton))),
                        ),
                        iced_aw::additional_menu::Item::new(
                            button(text("Move up one"))
                                .width(180)
                                .on_press(ProgramCommands::MovePlaylistUp(
                                    playlist.uniqueid.clone(),
                                    playlist.userorder,
                                ))
                                .style(iced::theme::Button::Custom(Box::new(SubMenuButton))),
                        ),
                        iced_aw::additional_menu::Item::new(
                            button(text("Move down one"))
                                .width(180)
                                .on_press(ProgramCommands::MovePlaylistDown(
                                    playlist.uniqueid.clone(),
                                    playlist.userorder,
                                ))
                                .style(iced::theme::Button::Custom(Box::new(SubMenuButton))),
                        ),
                        iced_aw::additional_menu::Item::new(
                            button(text(format!("delete {}", &playlist.title)))
                                .width(180)
                                .on_press(ProgramCommands::DeletePlaylist(
                                    playlist.uniqueid.clone(),
                                ))
                                .style(iced::theme::Button::Custom(Box::new(SubMenuButton))),
                        ),
                    ])
                    .max_width(180.0),
                );
                iced_aw::additional_menu::MenuBar::new(vec![dropdown])
                    .style(iced_aw::style::MenuBarStyle::Custom(Box::new(PungeMenu)))
                    .on_press(ProgramCommands::ChangeViewingPlaylist(
                        playlist.uniqueid.clone(),
                    ))
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
        btn.push(menu.into()); // the stupid button clips over the container border. so add this so it doesnt ...
        btn.push(self.horizontal_separator().into()); // separater between buttons and playlists :)
        btn.extend(playlist_buttons);
        container(row![Column::with_children(btn), self.vertical_separator()].spacing(5))
            .height(Length::Fill)
            // .style(iced::theme::Container::Custom(Box::new(
            //     ContainerWithBorder,
            // )))
            .into()
    }
    fn horizontal_separator(&self) -> quad::Quad {
        quad::Quad {
            quad_color: Color::from([0.5; 3]).into(),
            quad_border: Border {
                radius: [3.0; 4].into(),
                ..Default::default()
            },
            inner_bounds: InnerBounds::Ratio(0.98, 0.2),
            height: Length::Fixed(20.0),
            width: Length::Fixed(180.0),
            ..Default::default()
        }
        // is this like the only way to set it ..?
    }

    pub fn vertical_separator(&self) -> quad::Quad {
        quad::Quad {
            quad_color: Color::from([0.5; 3]).into(),
            quad_border: Border {
                radius: [3.0; 4].into(),
                ..Default::default()
            },
            inner_bounds: InnerBounds::Ratio(1.0, 1.0),
            height: Length::Fill,
            width: Length::Fixed(4.0),
            ..Default::default()
        }
        // is this like the only way to set it ..?
    }
}
