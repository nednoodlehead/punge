use crate::gui::widgets::hover_menu::create_hover_menu;
use crate::gui::widgets::row_overlay::OverlayButtons;
use iced::advanced::mouse;
use iced::advanced::{layout, renderer, widget::Tree, widget::Widget};
use iced::widget::{button, column, row, text, Column, Row};
use iced::Event;
use iced::{Border, Color, Element, Length, Point, Shadow, Size, Theme, Vector};

use crate::gui::style::button::punge_button_style;
// i dont think this is the best way to make this work. but passing in the Element from main.rs just caused issues
// like if it is held by the main struct, we cannot pass it into the overlay!?
pub fn create_menu<'a, 'b, Message, Theme, Renderer>(
    delete_msg: Message,
    quick_swap_msg: Message,
    add_to_msg: fn(String, String) -> Message,
    uuid_list: Vec<(String, String)>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    'b: 'a,
{
    column![
        button(text("delete")).on_press(delete_msg),
        button(text("Edit")),
        button(text("Quickswap 1")).on_press(quick_swap_msg),
        button(text("Add to..."))
    ]
    .into()
}

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
    selection_msg: fn(usize, bool, String) -> Message, // should be like: Selection(bool, String), "is 'uniqueid' selected" type of message
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
        selection_msg: fn(usize, bool, String) -> Message,
        add_to_msg: fn(String, String) -> Message,
        play_msg: fn(String) -> Message,
        move_song_up_msg: fn(String, usize) -> Message,
        move_song_down_msg: fn(String, usize) -> Message,
        edit_song_msg: fn(Option<String>) -> Message,
        mut uuid_list: Vec<(String, String)>,
        song_uuid: String,
    ) -> Self
    where
        <Theme as iced::widget::button::Catalog>::Class<'a>: From<
            Box<dyn Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a>,
        >,
    {
        // remove 'main'
        uuid_list.remove(0);
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
    // needs to have 4 children:
    // 1. the row itself
    // 2. the primary buttons for the overlay (edit, play, moveup...)
    // 3. the hover-button. so we can check if the cursor is above it, we show the submenu
    // 4. the sub-menu. for same reasons ^^
    fn children(&self) -> Vec<Tree> {
        let t: Element<Message, Theme, Renderer> = button(text("Add to...")).into();
        vec![
            Tree::new(&self.rowdata),
            Tree::new((&self.row_overlay)(
                self.delete_msg,
                self.quick_swap_msg,
                self.add_to_msg,
                self.play_msg,
                self.move_song_up_msg,
                self.move_song_down_msg,
                self.edit_song_msg,
                self.uuid_list.clone(),
                self.song_uuid.clone(),
                0,
            )),
            Tree::new(create_hover_menu::<Message, Theme, Renderer>(
                self.add_to_msg,
                self.uuid_list.clone(),
                String::from(""),
            )),
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
            show_sub_menu: false,
            sub_menu_spot: Point::default(),
            is_selected: false,
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
        style: &renderer::Style,
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
        let st: &RowState = tree.state.downcast_ref();
        if !st.show_bar {
            return None;
        }
        Some(
            OverlayButtons::new(
                tree,
                (self.row_overlay)(
                    self.delete_msg.clone(),
                    self.quick_swap_msg.clone(),
                    self.add_to_msg.clone(),
                    self.play_msg,
                    self.move_song_up_msg,
                    self.move_song_down_msg,
                    self.edit_song_msg,
                    self.uuid_list.clone(),
                    self.song_uuid.clone(),
                    self.row_num,
                )
                .into(),
                st.cursor_pos,
                self.row_num,
                self.add_to_msg.clone(),
                self.uuid_list.clone(),
                self.song_uuid.clone(),
            )
            .into(),
        )
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: iced::Event,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                // println!("current viewport: {:?}", &viewport);
                let st: &mut RowState = state.state.downcast_mut();
                if cursor.is_over(layout.bounds()) {
                    st.show_bar = true;

                    // we offset the viewport and the cursor position to place the cursor where it needs to be
                    // i found this out all on my own omg im so smart :3
                    let mut def_cursor = cursor.position().unwrap();
                    let actual_y_coord = (def_cursor.y - viewport.y) + 100.0; // 30 = approv def. length of button
                    def_cursor.y = actual_y_coord;
                    st.cursor_pos = def_cursor;
                    iced::event::Status::Captured
                } else {
                    st.show_bar = false;
                    iced::event::Status::Captured
                }
            }

            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                // for selecting rows and such
                if cursor.is_over(layout.bounds()) {
                    let st: &mut RowState = state.state.downcast_mut();
                    if st.is_selected {
                        st.is_selected = false;

                        // it would make sense to have a shell.publish and take the msg, add it to a list on the app, then when an
                        // action is done, do whatever to the contents of the list, then set the values all to false.
                        // but this stupid shell.publish stuff makes no sense..

                        // shell.publish((self.selection_msg)(self.row_num, false));
                    } else {
                        st.is_selected = true;
                        // shell.publish((self.selection_msg)(self.row_num, true));
                    }
                    iced::event::Status::Captured
                } else {
                    iced::event::Status::Ignored
                }
            }

            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                // it aint perfect by any means, but it works fairly well. we are going to leave it in!!
                let tmp_cursor = cursor.position();
                let st: &mut RowState = state.state.downcast_mut();
                match tmp_cursor {
                    None => return iced::event::Status::Ignored,
                    Some(tmp) => {
                        let overlayed = tmp.y - viewport.y + 30.0;
                        let mut new_layout = layout.bounds();
                        new_layout.y = new_layout.y - viewport.y + 30.0;
                        let m = iced::advanced::mouse::Cursor::Available(position);
                        if !m.is_over(new_layout) {
                            st.show_bar = false;
                            iced::event::Status::Captured
                        } else {
                            iced::event::Status::Ignored
                        }
                    }
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
    pub show_sub_menu: bool,
    pub sub_menu_spot: Point,
    pub is_selected: bool,
}
