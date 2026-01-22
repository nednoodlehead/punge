use iced::advanced::layout::Limits;
// the purpose of this widget is to be used as text for a playlist. It will be left-clickable and it will show the playlist
// and a right-click will show some playlist options (edit, move up, move down, duplicate, play)
use iced::advanced::{layout, renderer, widget::Tree, widget::Widget};
use iced::advanced::{mouse, Overlay};
use iced::{Element, Length, Point, Size, Vector};
use iced::{Event, Rectangle};

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
    button_overlay: Element<'a, Message, Theme, Renderer>,
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
        button_overlay: Element<'a, Message, Theme, Renderer>,
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
    for PlaylistButton<'_, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.button), Tree::new(&self.button_overlay)]
    }

    fn size(&self) -> Size<Length> {
        self.button.as_widget().size()
    }
    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.button, &self.button_overlay]);
    }
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.button
            .as_widget_mut()
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

    fn update(
        &mut self,
        state: &mut Tree,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) {
        let st = state.state.downcast_mut::<PlaylistButtonState>();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                if cursor.is_over(layout.bounds()) {
                    st.show_menu = true;
                    st.cursor_pos = cursor.position().unwrap();
                    st.cursor_pos.x = st.cursor_pos.x - 5.0;
                    st.cursor_pos.y = st.cursor_pos.y - 5.0;
                    shell.request_redraw();
                } else {
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if cursor.is_over(layout.bounds()) {
                    shell.publish(self.view_message.clone());
                }
            }
            _ => (),
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        _layout: layout::Layout<'b>,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        _translation: Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        let st: &mut PlaylistButtonState = tree.state.downcast_mut();
        if !st.show_menu {
            return None;
        }
        Some(iced::advanced::overlay::Element::new(Box::new(
            PlaylistButtonOverlay {
                tree: &mut tree.children[1],
                overlay: &mut self.button_overlay,
                state: st,
            },
        )))
    }
}

pub struct PlaylistButtonOverlay<'a, 'b, Message, Theme, Renderer>
where
    Message: 'b + Clone,
    Theme: iced::widget::button::Catalog,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    pub tree: &'b mut Tree,
    pub overlay: &'b mut Element<'a, Message, Theme, Renderer>,
    pub state: &'b mut PlaylistButtonState,
}

impl<'a, 'b, Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for PlaylistButtonOverlay<'a, 'b, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: iced::widget::button::Catalog + iced::widget::text::Catalog,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let node = self
            .overlay
            .as_widget_mut()
            .layout(&mut self.tree, renderer, &limits);
        // because there is the padding (fn create_playlist_button_menu) of 10px, we need to remove 10 from the postiiton
        node.move_to(self.state.cursor_pos)
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
            &self.tree,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        )
    }

    fn update(
        &mut self,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
    ) {
        // let st = self.tree.state.downcast_mut::<PlaylistButtonState>();
        match event {
            Event::Mouse(mouse::Event::CursorMoved { position: _ }) => {
                if self.state.show_menu {
                    let tmp = cursor.position();
                    match tmp {
                        None => (),
                        Some(_) => {
                            if !cursor.is_over(layout.bounds()) {
                                self.state.show_menu = false;
                            }
                        }
                    }
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                self.overlay.as_widget_mut().update(
                    &mut self.tree,
                    event,
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    &layout.bounds(),
                );
                // self.state.show_menu = false;
            }
            _ => {
                self.overlay.as_widget_mut().update(
                    &mut self.tree,
                    event,
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    &layout.bounds(),
                );
            }
        }
    }
}

// impl<'a, 'b, Message, Theme, Renderer> From<PlaylistButtonOverlay<'a, 'b, Message, Theme, Renderer>>
//     for iced::advanced::overlay::Element<'a, Message, Theme, Renderer>
// where
//     Message: 'a + Clone,
//     Theme: 'a + iced::widget::button::Catalog + iced::widget::text::Catalog,
//     Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
// {
//     fn from(overlay: PlaylistButtonOverlay<'a, 'b, Message, Theme, Renderer>) -> Self {
//         Self::new(Box::new(overlay))
//     }
// }
