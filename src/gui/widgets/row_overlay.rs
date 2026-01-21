use crate::gui::widgets::row::RowState;
use iced::advanced::layout::Limits;
use iced::advanced::{layout, renderer, widget::Tree, Overlay};
use iced::advanced::{mouse, overlay};
use iced::widget::button;
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
    pub hover_menu: fn(
        fn(String, String) -> Message,
        Vec<(String, String)>,
        String,
    ) -> Element<'a, Message, Theme, Renderer>,
    pub position: Point,
    pub row_num: usize,
    pub add_to_msg: fn(String, String) -> Message,
    pub uuid_list: Vec<(String, String)>,
    pub song_uuid: String,
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
        hover_menu: fn(
            fn(String, String) -> Message,
            Vec<(String, String)>,
            String,
        ) -> Element<'a, Message, Theme, Renderer>,
        position: Point,
        row_num: usize,
        add_to_msg: fn(String, String) -> Message,
        uuid_list: Vec<(String, String)>,
        song_uuid: String,
    ) -> Self {
        OverlayButtons {
            tree,
            overlay,
            hover_menu,
            position,
            row_num,
            add_to_msg,
            uuid_list,
            song_uuid,
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
        let limits = Limits::new(Size::ZERO, bounds);
        let node = layout::Node::with_children(
            bounds,
            vec![self.overlay.as_widget_mut().layout(
                &mut self.tree.children[1],
                renderer,
                &limits,
            )],
        );
        node.move_to(self.position)
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
    fn update(
        &mut self,
        event: &Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
    ) {
        match event {
            // ALSO: we are fixing #82 by refreshing the playlist (self.refresh_playlist() ) every time we create a message from that menu
            // so addtoplaylist, Delete, Quickswap, move up & down
            // it is sort of a hackjob, and to the 3 people worldwide reading this, if you want to, you are very welcome to make the menu
            // close in a different manner. I'm not sure how else to do it...
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                let st: &mut RowState = self.tree.state.downcast_mut();
                // need to get the bottom button...
                // this makes me upset
                // this is compete garbage and I know it. i wish iced had more docs on this stuff. all of the iced_aw
                // examples are insanely complex. also the api for passing widgets around is bad.
                // should it not be: parent widget is aware of all children, and all children reference the tree?
                // the tree is comprised of the widgets (and sub widgets)
                // .expand is our padding here btw
                // no invert is normal!
                if !st.invert_bar {
                    let lc = layout.children().next().unwrap().bounds().expand(10.0);
                    let top_left_corner = lc.y;
                    let y_spot = top_left_corner + lc.height - 40.0;
                    let x_spot = lc.width + lc.x - 10.0;
                    let mut top_of_btn = lc.position();
                    top_of_btn.y = top_of_btn.y + lc.height - 40.0;
                    // aprox size of the button at the bottom
                    let add_to_area = iced::Rectangle::new(top_of_btn, Size::new(80.0, 37.0));
                    let mut overlay_y = 0.0;
                    for _ in self.uuid_list.iter() {
                        // size of the menu.. depends on existing buttons
                        overlay_y += 42.5
                    }
                    let mut top_overlay = top_of_btn.clone();
                    top_overlay.x += 79.0;
                    let overlay_area =
                        iced::Rectangle::new(top_overlay, Size::new(150.0, overlay_y));
                    if add_to_area.contains(*position) || overlay_area.contains(*position) {
                        // we should show the sub menu
                        st.sub_menu_spot = Point::new(x_spot, y_spot);
                        st.show_sub_menu = true;
                    } else if !lc.contains(*position) {
                        // we are outside of the menus, stop showing them
                        st.show_bar = false;
                        st.show_sub_menu = false;
                    } else {
                        // we are inside the menus, but not over the overlay area / bottom button, show just the menu!
                        st.show_sub_menu = false;
                    }
                }
                // the bar is inverted because of where the cursor is in the viewport. so we invert it...
                else {
                    let lc = layout.children().next().unwrap().bounds().expand(10.0);
                    let top_left_corner = lc.y;
                    let open_hover_area = iced::Rectangle::new(
                        iced::Point::new(lc.x, top_left_corner),
                        Size::new(120.0, 37.0),
                    );
                    let mut overlay_y = 0.0;
                    for _ in self.uuid_list.iter() {
                        overlay_y += 42.5
                    }
                    let hover_menu_x = lc.x + 79.0;

                    let overlay_area = iced::Rectangle::new(
                        Point::new(hover_menu_x + 30.0, top_left_corner),
                        Size::new(150.0, overlay_y),
                    );

                    if open_hover_area.contains(*position) || overlay_area.contains(*position) {
                        st.sub_menu_spot = Point::new(lc.x + 120.0, top_left_corner + 10.0); // ???
                        st.show_sub_menu = true;
                    } else if !lc.contains(*position) {
                        st.show_bar = false;
                        st.show_sub_menu = false;
                    } else {
                        st.show_sub_menu = false;
                    }
                }
            }
            _ => self.overlay.as_widget_mut().update(
                &mut self.tree.children[1],
                event,
                layout.children().next().unwrap(),
                cursor,
                renderer,
                clipboard,
                shell,
                &layout.bounds(),
            ),
        }
    }
    fn overlay<'b>(
        &'b mut self,
        _layout: layout::Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let state: &mut RowState = self.tree.state.downcast_mut();
        if !state.show_sub_menu {
            return None;
        }
        Some(
            crate::gui::widgets::hover_menu::HoverMenu::new(
                &mut self.tree,
                self.hover_menu,
                self.add_to_msg,
                self.uuid_list.clone(),
                self.song_uuid.clone(),
            )
            .into(),
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
