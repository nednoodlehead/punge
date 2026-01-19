use crate::gui::style::button::punge_button_style;
use crate::gui::widgets::row::RowState;
use iced::advanced::layout::Limits;
use iced::advanced::mouse;
use iced::advanced::{layout, renderer, widget::Tree, Overlay};
use iced::widget::{button, column, text};
use iced::{Element, Event, Size};

pub fn create_hover_menu<'a, Message, Theme, Renderer>(
    add_to_msg: fn(String, String) -> Message,
    uuid_list: Vec<(String, String)>,
    song_uuid: String,
) -> Element<'a, Message, Theme, Renderer>
where
    <Theme as iced::widget::button::Catalog>::Class<'a>:
        From<Box<dyn Fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style + 'a>>,
    Message: 'a + Clone,
    Theme: 'a + iced::widget::button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    let mut col = column![];
    if uuid_list.len() == 0 {
        col = col.push(button(text("No playlists :(")));
    } else {
        for item in uuid_list {
            col = col.push(
                // .1 = name .0 = uuid
                button(text(item.1))
                    .width(110)
                    .on_press((add_to_msg)(item.0.clone(), song_uuid.clone()))
                    .style(|_, status| punge_button_style(status)),
            );
        }
    };
    col.into()
}

pub struct HoverMenu<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    pub tree: &'a mut Tree,
    // pub state: &'a mut MenuState,
    pub content: Element<'a, Message, Theme, Renderer>,
    // pub add_to_msg: Option<fn(&str) -> Message>,
    // pub uuid_list: Vec<(&'a str, &'a str)>,
}

impl<'a, Message, Theme, Renderer> HoverMenu<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    pub fn new(
        tree: &'a mut Tree,
        menu: fn(
            fn(String, String) -> Message,
            Vec<(String, String)>,
            String,
        ) -> Element<'a, Message, Theme, Renderer>,
        add_to_msg: fn(String, String) -> Message,
        uuid_list: Vec<(String, String)>,
        song_uuid: String,
    ) -> Self {
        HoverMenu {
            tree,
            // state,
            content: (menu)(add_to_msg, uuid_list, song_uuid),
        }
    }
}

impl<'a, Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for HoverMenu<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = Limits::new(Size::ZERO, bounds);
        let node =
            self.content
                .as_widget_mut()
                .layout(&mut self.tree.children[2], renderer, &limits);
        let st: &RowState = self.tree.state.downcast_ref();
        node.move_to(st.sub_menu_spot)
    }
    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            &self.tree.children[2],
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
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
        self.content.as_widget_mut().update(
            &mut self.tree.children[2],
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
impl<'a, Message, Theme, Renderer> From<HoverMenu<'a, Message, Theme, Renderer>>
    for iced::advanced::overlay::Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + button::Catalog + iced::widget::text::Catalog,
    Renderer: 'a + iced::advanced::Renderer + iced::advanced::text::Renderer,
{
    fn from(overlay: HoverMenu<'a, Message, Theme, Renderer>) -> Self {
        Self::new(Box::new(overlay))
    }
}
