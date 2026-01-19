use iced::widget::slider;
use iced::widget::slider::{Status, Style};
use iced::{border::Radius, Background, Border, Color};

pub fn scrubber_style(status: Status) -> Style {
    match status {
        Status::Hovered => Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color::BLACK),
                    Background::Color(Color::from_rgb(0.410, 0.410, 0.410)),
                ),
                width: 4.0,
                border: Border {
                    color: Color::BLACK,
                    width: 1.0,
                    radius: Radius::from(10),
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 4.0 },
                background: Background::Color(Color::from_rgb(0.175, 0.175, 0.175)),
                border_width: 10.0,
                border_color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        },
        Status::Active => Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color::BLACK),
                    Background::Color(Color::from_rgb(0.410, 0.410, 0.410)),
                ),
                width: 4.0,
                border: Border {
                    radius: Radius::from(10),
                    ..Default::default()
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 4.0 },
                border_width: 10.0,
                background: Background::Color(Color::from_rgb(0.175, 0.175, 0.175)),
                border_color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        },
        Status::Dragged => Style {
            rail: slider::Rail {
                width: 4.0,
                backgrounds: (
                    Background::Color(Color::BLACK),
                    Background::Color(Color::from_rgb(0.410, 0.410, 0.410)),
                ),
                border: Border {
                    radius: Radius::from(10),
                    ..Default::default()
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 7.0 },
                background: Background::Color(Color::BLACK),
                border_width: 0.0,
                border_color: Color::BLACK,
            },
        },
    }
}
