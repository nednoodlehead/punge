use crate::gui::style::button::punge_button_style;
use crate::gui::widgets::row_overlay::OverlayButtons;
use iced::advanced::mouse;
use iced::advanced::{layout, renderer, widget::Tree, widget::Widget};
use iced::widget::{button, column, row, text, Column, Row, Themer};
use iced::Event;
use iced::{Border, Color, Element, Length, Point, Shadow, Size, Theme, Vector};

#[derive(Debug, Clone)]
pub struct RowData {
    pub title: String,
    pub author: String,
    pub album: String,
    pub row_num: usize,
    pub uniqueid: String,
}

pub struct RowWidget<'a, Message, Theme, Renderer>
where
    // <Theme as iced::widget::button::Catalog>::Class<'a>:
    //     From<Box<dyn Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a>>,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: iced::widget::text::Catalog + iced::widget::button::Catalog,
{
    rowdata: Element<'a, Message, Theme, Renderer>,
    row_overlay: fn(
        fn(String) -> Message,
        fn(String) -> Message,
        fn(String, String) -> Message,
        fn(String) -> Message,
        fn(String, usize) -> Message,
        fn(String, usize) -> Message,
        fn(Option<String>) -> Message,
        Vec<(String, String)>,
        String,
        usize,
    ) -> Element<'a, Message, Theme, Renderer>,
    delete_msg: fn(String) -> Message,
    quick_swap_msg: fn(String) -> Message,
    selection_msg: fn(usize, bool) -> Message, // should be like: Selection(bool, String), "is 'uniqueid' selected" type of message
    add_to_msg: fn(String, String) -> Message,
    play_msg: fn(String) -> Message,
    move_song_up_msg: fn(String, usize) -> Message,
    move_song_down_msg: fn(String, usize) -> Message,
    edit_song_msg: fn(Option<String>) -> Message,
    uuid_list: Vec<(String, String)>,
    row_num: usize,
    is_selected: bool,
    show_menu: bool,
    cursor_pos: Point,
    song_uuid: String,
}

impl<'a, Message, Theme, Renderer> RowWidget<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    pub fn new(
        title: &'a String,
        author: &'a String,
        album: &'a String,
        row_num: usize,
        delete_msg: fn(String) -> Message,
        quick_swap_msg: fn(String) -> Message,
        selection_msg: fn(usize, bool) -> Message,
        add_to_msg: fn(String, String) -> Message,
        play_msg: fn(String) -> Message,
        move_song_up_msg: fn(String, usize) -> Message,
        move_song_down_msg: fn(String, usize) -> Message,
        edit_song_msg: fn(Option<String>) -> Message,
        uuid_list: Vec<(String, String)>,
        song_uuid: String,
    ) -> Self
    where
        <Theme as iced::widget::button::Catalog>::Class<'a>: From<
            Box<dyn Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a>,
        >,
    {
        // .width(30)
        // .clip(true)
        // .padding(0),
        let mut rowdata = row![button(text(row_num.to_string()))
            .on_press((play_msg)(song_uuid.clone()))
            .width(30)
            .clip(true)
            .padding(0)
            .style(|_t, status| punge_button_style(status))];
        for disp_text in [title, author, album] {
            if disp_text.len() < 30 {
                rowdata = rowdata.push(text(disp_text).width(350));
            } else if disp_text.len() > 50 {
                rowdata = rowdata.push(text(disp_text).size(8).width(350));
            } else {
                // text that is sort of large, but not huge like ^^
                rowdata = rowdata.push(text(disp_text).size(13).width(350));
            }
        }
        RowWidget {
            rowdata: rowdata.spacing(10).into(),
            row_overlay: crate::gui::persistent::create_whole_menu,
            is_selected: false,
            delete_msg,
            quick_swap_msg,
            selection_msg,
            add_to_msg,
            play_msg,
            move_song_up_msg,
            move_song_down_msg,
            edit_song_msg,
            uuid_list,
            row_num,
            show_menu: false,
            cursor_pos: Point::default(), // only is updated in 'on_event'
            song_uuid,
        }
    }
}

impl<'a, Message, Theme, Renderer> From<RowWidget<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    fn from(rowwidget: RowWidget<'a, Message, Theme, Renderer>) -> Self {
        Self::new(rowwidget)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for RowWidget<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.rowdata),
            Tree::new((&self.row_overlay)(
                self.delete_msg.clone(),
                self.quick_swap_msg.clone(),
                self.add_to_msg.clone(),
                self.play_msg.clone(),
                self.move_song_up_msg.clone(),
                self.move_song_down_msg.clone(),
                self.edit_song_msg.clone(),
                self.uuid_list.clone(),
                self.song_uuid.clone(),
                0, // doesnt matter for the `children` part
            )), // all of a sudden i need type params !?
        ]
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(100.0),
            height: Length::Fixed(35.0),
        }
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(RowState {
            cursor_pos: self.cursor_pos,
            show_bar: self.show_menu,
        })
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        layout::Node::with_children(
            Size {
                width: 1000.0,
                height: 35.0,
            },
            vec![self
                .rowdata
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)],
        )
    }
    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        // varying cell colors
        let cell_color = if self.is_selected {
            Color {
                r: 0.2,
                g: 0.2,
                b: 0.8,
                a: 0.9,
            }
        } else if self.row_num % 2 == 0 {
            Color {
                r: 0.3,
                g: 0.3,
                b: 0.3,
                a: 1.0,
            }
        } else {
            Color {
                r: 0.15,
                g: 0.15,
                b: 0.15,
                a: 1.0,
            }
        };
        // create the cell color

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border::default(),
                ..renderer::Quad::default()
            },
            cell_color,
        );
        self.rowdata.as_widget().draw(
            // <-- draws all of them
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: Color::WHITE,
            },
            layout.children().next().unwrap(),
            cursor,
            &viewport,
        );
    }
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let state = tree.state.downcast_mut::<RowState>();
        if !state.show_bar {
            return None;
        }
        Some(
            OverlayButtons::new(
                tree,
                (self.row_overlay)(
                    self.delete_msg.clone(),
                    self.quick_swap_msg.clone(),
                    self.add_to_msg.clone(),
                    self.play_msg.clone(),
                    self.move_song_up_msg.clone(),
                    self.move_song_down_msg.clone(),
                    self.edit_song_msg.clone(),
                    self.uuid_list.clone(),
                    self.song_uuid.clone(),
                    self.row_num,
                )
                .into(),
                self.cursor_pos,
                self.row_num,
            )
            .into(),
        )
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced::Event,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        // alows the button to actually do something
        if cursor.is_over(
            // this is the button!!
            layout
                .children()
                .next()
                .unwrap()
                .children()
                .next()
                .unwrap()
                .bounds(),
        ) {
            self.rowdata.as_widget_mut().on_event(
                &mut tree.children[0],
                event.clone(),
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
            // shell.publish((self.play_msg)(self.song_uuid.clone()));
            return iced::event::Status::Captured;
        }
        let state = tree.state.downcast_mut::<RowState>();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                if cursor.is_over(layout.bounds()) {
                    state.show_bar = true;
                    // self.show_menu = true; // will make the menu appear in the first place
                    // we offset the viewport and the cursor position to place the cursor where it needs to be
                    // i found this out all on my own omg im so smart :3
                    let mut def_cursor = cursor.position().unwrap();
                    let actual_y_coord = (def_cursor.y - viewport.y) + 100.0; // 30 = approv def. length of button
                    def_cursor.y = actual_y_coord;
                    state.cursor_pos = def_cursor;
                    println!("set cursor to: {:#?}", &state.cursor_pos);
                    iced::event::Status::Captured
                } else {
                    // self.show_menu = false; // makes it so there is only one menu open at a time
                    iced::event::Status::Captured
                }
            }

            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                // for selecting rows and such
                if cursor.is_over(layout.bounds()) {
                    if self.is_selected {
                        println!("but not here");
                        self.is_selected = false;
                        // shell.publish((self.selection_msg)(self.row_num, false));

                        // it would make sense to have a shell.publish and take the msg, add it to a list on the app, then when an
                        // action is done, do whatever to the contents of the list, then set the values all to false.
                        // but this stupid shell.publish stuff makes no sense..

                        // shell.publish((self.selection_msg)(self.row_num, false));
                        iced::event::Status::Captured
                    } else {
                        println!("right on here");
                        self.is_selected = true;
                        // shell.publish((self.selection_msg)(self.row_num, true));
                        iced::event::Status::Captured
                    }
                } else {
                    iced::event::Status::Ignored
                }
            }

            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                // it aint perfect by any means, but it works fairly well. we are going to leave it in!!
                if state.show_bar {
                    let tmp_cursor = cursor.position();
                    match tmp_cursor {
                        None => return iced::event::Status::Ignored,
                        Some(_) => {
                            // should be the menu...?
                            let mut new_layout = layout.children().next().unwrap().bounds();
                            new_layout.y = new_layout.y - viewport.y + 30.0;
                            let m = iced::advanced::mouse::Cursor::Available(position);
                            if !m.is_over(new_layout) {
                                println!("break!");
                                state.show_bar = false;
                                // self.show_menu = false;
                                iced::event::Status::Captured
                            } else {
                                iced::event::Status::Ignored
                            }
                        }
                    }
                } else {
                    iced::event::Status::Ignored
                }
            }
            _ => iced::event::Status::Ignored,
        }
    }
}

#[derive(Debug)]
pub struct RowState {
    pub cursor_pos: Point,
    pub show_bar: bool,
}
