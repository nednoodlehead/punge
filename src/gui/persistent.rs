// this file is for elements that are persistent across pages
// such as the 'current playing' bar at the bottom, and the buttons at the top to change pages
use crate::gui::messages::{Page, ProgramCommands};
use crate::gui::start::App;
use crate::gui::style::button::{
    just_text, menu_button_style, playlist_text_style, punge_button_style, sub_menu_button_style,
};
use crate::gui::style::container::bottom_bar_container;
use crate::gui::style::menu::punge_menu_style;
use crate::gui::style::scrubber::scrubber_style;
use crate::gui::style::volume::volume_style;
use iced::widget::{button, column, container, horizontal_space, row, slider, text, Column, Image};
use iced::{Alignment, Element};
use iced_aw::menu::{Item, Menu};
use iced_aw::widgets::quad;
use iced_aw::widgets::InnerBounds;

pub fn create_whole_menu<'a, Message, Theme, Renderer>(
    delete_msg: fn(String) -> Message,
    quick_swap: fn(String) -> Message,
    add_to_msg: fn(String, String) -> Message,
    play_msg: fn(String) -> Message,
    move_song_up_msg: fn(String, usize) -> Message,
    move_song_down_msg: fn(String, usize) -> Message,
    edit_song_msg: fn(Option<String>) -> Message,
    uuid_list: Vec<(String, String)>,
    song_uuid: String,
    row_num: usize,
) -> Element<'a, Message, Theme, Renderer>
where
    <Theme as iced::widget::button::Catalog>::Class<'a>:
        From<Box<dyn Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a>>,
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    let mut col = column![
        button(text("Play!"))
            .on_press((play_msg)(song_uuid.clone()))
            .style(|_t, status| punge_button_style(status))
            .width(110),
        button(text("Edit"))
            .on_press((edit_song_msg)(Some(song_uuid.clone())))
            .style(|_t, status| punge_button_style(status))
            .width(110),
        button(text("Quickswap"))
            .on_press((quick_swap)(song_uuid.clone()))
            .style(|_t, status| punge_button_style(status))
            .width(110),
        button(text("Move up"))
            .on_press((move_song_up_msg)(song_uuid.clone(), row_num))
            .style(|_t, status| punge_button_style(status))
            .width(110),
        button(text("Move down"))
            .on_press((move_song_down_msg)(song_uuid.clone(), row_num))
            .style(|_t, status| punge_button_style(status))
            .width(110),
        button(text("Delete!"))
            .on_press((delete_msg)(song_uuid.clone()))
            .style(|_t, status| punge_button_style(status))
            .width(110),
    ];
    for (uuid, title) in uuid_list {
        col = col.push(
            button(text(format!("Add to: {}", &title)))
                .on_press((add_to_msg)(uuid, song_uuid.clone()))
                .style(|_t, status| punge_button_style(status))
                .width(110),
        )
    }
    col.into()
}

impl App {
    pub fn render_bottom_bar(&self) -> Element<'static, ProgramCommands> {
        let curr_song = self.current_song.load();
        let search_container = container(row![
            iced::widget::text_input("GoTo closest match", self.search.as_str())
                .on_input(ProgramCommands::UpdateSearch)
                .width(iced::Length::Fixed(200.0)),
            button(text("Confirm"))
                .on_press(ProgramCommands::GoToSong)
                .style(|_theme, status| punge_button_style(status))
        ]);
        container(
            row![
                Image::new(curr_song.thumbnail.clone())
                    .width(100)
                    .height(100),
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
                        button(text("<----"))
                            .style(|_theme, status| just_text(status))
                            .on_press(ProgramCommands::SkipBackwards),
                        button(if self.is_paused {
                            Image::new("./img/punge_play_new.png")
                        } else {
                            Image::new("./img/punge_pause_new.png")
                        })
                        .style(|_theme, status| just_text(status))
                        .height(50)
                        .width(50)
                        .on_press(ProgramCommands::PlayToggle),
                        button(text("---->"))
                            .on_press(ProgramCommands::SkipForwards)
                            .style(|_theme, status| just_text(status)),
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
                        .style(|_theme, status| scrubber_style(status)), // scrubberstyle
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
                        .style(|_style, status| just_text(status))
                        .on_press(ProgramCommands::ShuffleToggle),
                        slider(0..=30, self.volume, ProgramCommands::VolumeChange)
                            .width(150)
                            .style(|_theme, status| volume_style(status))
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
        .style(|_status| bottom_bar_container())
        .into()
    }
    pub fn render_buttons_side(&self, ignore: Page) -> Element<'static, ProgramCommands> {
        // let playlist_add_to_menu = Item::with_menu(
        //     button(text("Add to...                     ->")) // this is mad goofy lol
        //         .width(180)
        //         .style(|_theme, status| sub_menu_button_style(status))
        //         .on_press(ProgramCommands::Debug),
        //     Menu::new(
        //         self.user_playlists
        //             .iter()
        //             .map(|p| {
        //                 Item::new(
        //                     button(text(p.title.clone()))
        //                         .style(|_t, status| sub_menu_button_style(status))
        //                         .width(150)
        //                         .on_press(ProgramCommands::AddToPlaylist(p.uniqueid.clone())),
        //                 )
        //             })
        //             .collect(),
        //     )
        //     .max_width(150.0)
        //     .offset(10.0),
        // );
        // let menu = iced_aw::menu_bar!((
        //     button("Edit song")
        //         .style(|_t, status| sub_menu_button_style(status))
        //         .on_press(ProgramCommands::Debug),
        //     Menu::new(vec![
        //         Item::new(
        //             button(text("Full Edit"))
        //                 .on_press(ProgramCommands::OpenSongEditPage)
        //                 .style(|_t, status| sub_menu_button_style(status))
        //                 .width(180)
        //         ),
        //         Item::new(
        //             button(text("Swap Title & Author"))
        //                 .style(|_t, status| sub_menu_button_style(status))
        //                 .on_press(ProgramCommands::QuickSwapTitleAuthor)
        //                 .width(180),
        //         ),
        //         Item::new(
        //             button(text("Delete!!"))
        //                 .style(|_t, status| sub_menu_button_style(status))
        //                 .on_press(ProgramCommands::DeleteSong)
        //                 .width(180)
        //         ),
        //         playlist_add_to_menu,
        //     ])
        //     .offset(0.0)
        //     .max_width(180.0)
        // ))
        // .style(|_t, status| punge_menu_style(status));

        let mut all_playlists_but_main = self.user_playlists.clone();
        // user should always have the 'main' playlist

        // all_playlists_but_main.remove(0);
        // let playlist_buttons: Vec<Element<ProgramCommands>> = self
        //     .user_playlists
        //     .iter()
        //     .map(|playlist| {
        //         let dropdown = iced_aw::additional_menu::Item::with_menu(
        //             button(text(playlist.title.clone()))
        //                 .style(|_t, status| playlist_text_style(status)),
        //             iced_aw::additional_menu::Menu::new(vec![
        //                 iced_aw::additional_menu::Item::new(
        //                     button(text("Edit"))
        //                         .on_press(ProgramCommands::OpenPlaylistEditPage(playlist.clone()))
        //                         .width(180)
        //                         .style(|_t, status| sub_menu_button_style(status)),
        //                 ),
        //                 iced_aw::additional_menu::Item::new(
        //                     button(text("Duplicate?"))
        //                         .width(180)
        //                         .style(|_t, status| sub_menu_button_style(status)),
        //                 ),
        //                 iced_aw::additional_menu::Item::new(
        //                     button(text("Move up one"))
        //                         .width(180)
        //                         .on_press(ProgramCommands::MovePlaylistUp(
        //                             playlist.uniqueid.clone(),
        //                             playlist.userorder,
        //                         ))
        //                         .style(|_t, status| sub_menu_button_style(status)),
        //                 ),
        //                 iced_aw::additional_menu::Item::new(
        //                     button(text("Move down one"))
        //                         .width(180)
        //                         .on_press(ProgramCommands::MovePlaylistDown(
        //                             playlist.uniqueid.clone(),
        //                             playlist.userorder,
        //                         ))
        //                         .style(|_t, status| sub_menu_button_style(status)),
        //                 ),
        //                 iced_aw::additional_menu::Item::new(
        //                     button(text(format!("delete {}", &playlist.title)))
        //                         .width(180)
        //                         .on_press(ProgramCommands::DeletePlaylist(
        //                             playlist.uniqueid.clone(),
        //                         ))
        //                         .style(|_t, status| sub_menu_button_style(status)),
        //                 ),
        //             ])
        //             .max_width(180.0),
        //         );
        //         iced_aw::additional_menu::MenuBar::new(vec![dropdown])
        //             .style(|_t, status| punge_menu_style(status))
        //             .on_press(ProgramCommands::ChangeViewingPlaylist(
        //                 playlist.uniqueid.clone(),
        //             ))
        //             .into()
        //     })
        //     .collect();
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
                    button(text(*txt))
                        .style(|_t, status| menu_button_style(status))
                        .into()
                } else {
                    button(text(*txt))
                        .style(|_t, status| menu_button_style(status))
                        .on_press(ProgramCommands::ChangePage(*page))
                        .into()
                }
            })
            .collect();
        // btn.push(menu.into()); // the stupid button clips over the container border. so add this so it doesnt ...
        // btn.push(self.horizontal_separator().into()); // separater between buttons and playlists :)
        // btn.extend(playlist_buttons);
        container(row![Column::with_children(btn), text("spacing")].spacing(5))
            .height(iced::Length::Fill)
            // .style(iced::theme::Container::Custom(Box::new(
            //     ContainerWithBorder,
            // )))
            .into()
    }
    fn horizontal_separator(&self) -> quad::Quad {
        quad::Quad {
            quad_color: iced_core::Background::Color(iced_core::Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            }),
            quad_border: iced_core::Border {
                radius: [3.0; 4].into(),
                ..Default::default()
            },
            inner_bounds: InnerBounds::Ratio(0.98, 0.2),
            height: iced_core::Length::Fixed(20.0),
            width: iced_core::Length::Fixed(180.0),
            ..Default::default()
        }
        // is this like the only way to set it ..?
    }

    pub fn vertical_separator(&self) -> quad::Quad {
        quad::Quad {
            quad_color: iced_core::Background::Color(iced_core::Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            }),
            quad_border: iced_core::Border {
                radius: [3.0; 4].into(),
                ..Default::default()
            },
            inner_bounds: InnerBounds::Ratio(1.0, 1.0),
            height: iced_core::Length::Fill,
            width: iced_core::Length::Fixed(4.0),
            ..Default::default()
        }
        // is this like the only way to set it ..?
    }
}
