use iced::Theme;
use iced_core::{Background, Color, Padding};

pub fn punge_menu_style(_theme: &Theme) -> iced_aw::menu::Appearance {
    iced_aw::menu::Appearance {
        bar_background: Background::Color(Color::new(0.5, 0.5, 0.5, 0.0)),
        menu_background: Background::Color(Color::new(0.2, 0.2, 0.2, 1.0)),
        bar_background_expand: Padding::new(5.0),
        ..Default::default()
    }
}
