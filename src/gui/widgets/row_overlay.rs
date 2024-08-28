use crate::gui::widgets::row::{RowState};
use iced::advanced::layout::Limits;
use iced::advanced::{layout, renderer, widget::Tree, widget::Widget, Overlay};
use iced::advanced::{mouse};
use iced::widget::{button};
use iced::{Element, Event, Point, Size};

pub struct OverlayButtons<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: button::Catalog,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    pub tree: &'a mut Tree,
    // maybe change this at some point idk.. having both .overlay and .overlay()??
    pub overlay: Element<'a, Message, Theme, Renderer>,
    pub position: Point,
    pub row_num: usize,
}

impl<'a, Message, Theme, Renderer> OverlayButtons<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    pub fn new(
        tree: &'a mut Tree,
        overlay: Element<'a, Message, Theme, Renderer>,
        position: Point,
        row_num: usize,
    ) -> Self {
        OverlayButtons {
            tree,
            // state,
            overlay,
            position,
            row_num,
        }
    }
}

impl<'a, Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for OverlayButtons<'a, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: button::Catalog + iced::widget::text::Catalog,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let state = self.tree.state.downcast_ref::<RowState>();
        let limits = Limits::new(Size::ZERO, bounds);
        let node = layout::Node::with_children(
            Size {
                width: 110.0,
                height: 300.0,
            },
            vec![self
                .overlay
                .as_widget()
                .layout(&mut self.tree.children[1], renderer, &limits)],
        );
        node.move_to(state.cursor_pos)
    }
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let lay_1 = layout.children().next().unwrap();
        self.overlay.as_widget().draw(
            &self.tree.children[1],
            renderer,
            theme,
            style,
            lay_1,
            cursor,
            &layout.bounds(),
        );
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
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }
}

impl<'a, Message, Theme, Renderer> From<OverlayButtons<'a, Message, Theme, Renderer>>
    for iced::advanced::overlay::Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn from(overlay: OverlayButtons<'a, Message, Theme, Renderer>) -> Self {
        Self::new(Box::new(overlay))
    }
}
