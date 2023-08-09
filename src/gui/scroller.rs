use iced::Theme;
use iced::{Color, BorderRadius};
use iced::widget::container::Appearance;
use iced::widget::scrollable::{Scrollable, Scroller, Scrollbar};

pub struct CustomScroll;
impl From<CustomScroll> for iced::theme::Scrollable {
    fn from(value: CustomScroll) -> Self {
        iced::theme::Scrollable::Custom(Box::new(value))
    }
}

impl iced::widget::scrollable::StyleSheet for CustomScroll {
    type Style = Theme;
    fn active(&self, style: &Self::Style) -> Scrollbar {
        Scrollbar {
            background: None,
            border_radius: BorderRadius::from(10.0),
            border_width: 10.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: Color::from_rgb(0.23, 0.34, 0.10),
                border_radius: BorderRadius::from(10.0),
                border_width: 10.0,
                border_color: Color::BLACK
            }
        }
    }
    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        Scrollbar {
            background: None,
            border_radius: BorderRadius::from(10.0),
            border_width: 10.0,
            border_color: Color::BLACK,
            scroller: Scroller {
                color: Color::from_rgb(0.23, 0.34, 0.10),
                border_radius: BorderRadius::from(10.0),
                border_width: 10.0,
                border_color: Color::BLACK
            }
        }
    }
}

pub struct ScrollerContainer;

impl iced::widget::container::StyleSheet for ScrollerContainer {
    type Style = Theme;
    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance {
            text_color: None,
            background: None,
            border_radius: BorderRadius::from(10.0),
            border_width: 1.0,
            border_color: Color::BLACK
        }
    }
}