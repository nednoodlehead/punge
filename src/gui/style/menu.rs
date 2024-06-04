use iced_aw::menu::StyleSheet;
use iced_core::border::Radius;
use iced_core::{Background, Border, Color, Padding, Shadow, Vector};

pub struct PungeMenu;

impl StyleSheet for PungeMenu {
    type Style = iced::Theme;
    fn appearance(&self, style: &Self::Style) -> iced_aw::menu::Appearance {
        iced_aw::menu::Appearance {
            bar_background: Background::Color(Color::new(0.5, 0.5, 0.5, 0.0)),
            menu_background: Background::Color(Color::new(0.2, 0.2, 0.2, 1.0)),
            bar_background_expand: Padding::new(5.0),
            ..Default::default()
        }
    }
}
