use iced::advanced::layout::Limits;
// the purpose of this widget is to be used as text for a playlist. It will be left-clickable and it will show the playlist
// and a right-click will show some playlist options (edit, move up, move down, duplicate, play)
use iced::advanced::{layout, renderer, widget::Tree, widget::Widget};
use iced::advanced::{mouse, Overlay};
use iced::widget::{button, column, row, text, Button, Column, Row, Themer};
use iced::Event;
use iced::{Border, Color, Element, Length, Point, Shadow, Size, Theme, Vector};

pub struct PlaylistButtonState {
    show_menu: bool,
    cursor_pos: Point,
}

pub struct PlaylistButton<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: iced::widget::text::Catalog + iced::widget::button::Catalog,
{
    button: Element<'a, Message, Theme, Renderer>,
    view_message: Message,
    button_overlay: fn(
        Message, // edit
        Message, // move up
        Message, // move down
        Message, // duplicate
        Message, // play
    ) -> Element<'a, Message, Theme, Renderer>,
    edit_message: Message,      // edit
    move_up_message: Message,   // move up
    move_down_message: Message, // move down
    dupe_message: Message,      // duplicate
    play_message: Message,      // play
    cursor_pos: Point,
}

impl<'a, Message, Theme, Renderer> PlaylistButton<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    pub fn new(
        button: Element<'a, Message, Theme, Renderer>,
        view_message: Message,
        button_overlay: fn(
            Message, // edit
            Message, // move up
            Message, // move down
            Message, // duplicate
            Message, // play
        ) -> Element<'a, Message, Theme, Renderer>,
        edit_message: Message,      // edit
        move_up_message: Message,   // move up
        move_down_message: Message, // move down
        dupe_message: Message,      // duplicate
        play_message: Message,      // play
    ) -> Self
    where
        <Theme as iced::widget::button::Catalog>::Class<'a>: From<
            Box<dyn Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a>,
        >,
    {
        PlaylistButton {
            button,
            view_message,
            button_overlay,
            edit_message,
            move_up_message,
            move_down_message,
            dupe_message,
            play_message,
            cursor_pos: Point::default(),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<PlaylistButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    fn from(playlist_button: PlaylistButton<'a, Message, Theme, Renderer>) -> Self {
        Self::new(playlist_button)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for PlaylistButton<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![
            Tree::new(&self.button),
            Tree::new(&(self.button_overlay)(
                self.edit_message.clone(),
                self.move_up_message.clone(),
                self.move_down_message.clone(),
                self.dupe_message.clone(),
                self.play_message.clone(),
            )),
        ]
    }

    fn size(&self) -> Size<Length> {
        self.button.as_widget().size()
    }
    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.button
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.button.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        )
    }
    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(PlaylistButtonState {
            show_menu: false,
            cursor_pos: self.cursor_pos,
        })
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        let st = state.state.downcast_mut::<PlaylistButtonState>();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                if cursor.is_over(layout.bounds()) {
                    st.show_menu = true;
                    st.cursor_pos = cursor.position().unwrap();
                    iced::event::Status::Captured
                } else {
                    iced::event::Status::Ignored
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(layout.bounds()) {
                    shell.publish(self.view_message.clone());
                    return iced::event::Status::Captured;
                }
                return iced::event::Status::Ignored;
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                if st.show_menu {
                    let tmp = cursor.position();
                    match tmp {
                        None => return iced::event::Status::Ignored,
                        Some(_) => {
                            if !cursor.is_over(layout.children().next().unwrap().bounds()) {
                                st.show_menu = false;
                                return iced::event::Status::Captured;
                            }
                            iced::event::Status::Ignored
                        }
                    }
                } else {
                    iced::event::Status::Ignored
                }
            }
            _ => iced::event::Status::Ignored,
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        _layout: layout::Layout<'_>,
        _renderer: &Renderer,
        _translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let st: &mut PlaylistButtonState = tree.state.downcast_mut();
        if !st.show_menu {
            return None;
        }
        Some(
            PlaylistButtonOverlay {
                tree,
                overlay: ((self.button_overlay)(
                    self.edit_message.clone(),
                    self.move_up_message.clone(),
                    self.move_down_message.clone(),
                    self.dupe_message.clone(),
                    self.play_message.clone(),
                )),
                postion: self.cursor_pos,
            }
            .into(),
        )
    }
}

pub struct PlaylistButtonOverlay<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: button::Catalog,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    pub tree: &'a mut Tree,
    pub overlay: Element<'a, Message, Theme, Renderer>,
    pub postion: Point,
}

impl<'a, Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for PlaylistButtonOverlay<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: button::Catalog + iced::widget::text::Catalog,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let st = self.tree.state.downcast_mut::<PlaylistButtonState>();
        let limits = Limits::new(Size::ZERO, bounds);
        let node = self
            .overlay
            .as_widget()
            .layout(&mut self.tree.children[1], renderer, &limits);
        node.move_to(st.cursor_pos)
    }
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.overlay.as_widget().draw(
            &self.tree.children[1],
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        )
    }
    fn on_event(
        &mut self,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
    ) -> iced::advanced::graphics::core::event::Status {
        self.overlay.as_widget_mut().on_event(
            &mut self.tree.children[1],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }
}

impl<'a, Message, Theme, Renderer> From<PlaylistButtonOverlay<'a, Message, Theme, Renderer>>
    for iced::advanced::overlay::Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn from(overlay: PlaylistButtonOverlay<'a, Message, Theme, Renderer>) -> Self {
        Self::new(Box::new(overlay))
    }
}
