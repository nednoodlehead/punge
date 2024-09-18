use iced::advanced::{renderer, Widget};
use iced::{Border, Color, Element, Length};

pub struct Separator {
    pub width: Length,
    pub height: Length,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Separator
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    fn size(&self) -> iced::Size<Length> {
        iced::Size::new(self.width, self.height)
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let limits = limits.width(self.width).height(self.height);
        iced::advanced::layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: Border {
                    radius: [3.0; 4].into(),
                    ..Default::default()
                },
                ..renderer::Quad::default()
            },
            Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            },
        )
    }
}
impl<'a, Message, Theme, Renderer> From<Separator> for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + iced::widget::text::Catalog + iced::widget::button::Catalog,
    Message: 'a + Clone,
{
    fn from(rowwidget: Separator) -> Self {
        Self::new(rowwidget)
    }
}
