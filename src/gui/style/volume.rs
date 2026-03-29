use iced::border::Radius;
use iced::widget::slider;
use iced::widget::slider::{Status, Style};
use iced::{Background, Color};

pub fn volume_style(status: Status) -> Style {
    match status {
        Status::Active => Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color {
                        r: 0.85,
                        g: 0.85,
                        b: 0.85,
                        a: 1.0,
                    }),
                    Background::Color(Color {
                        r: 0.310,
                        g: 0.310,
                        b: 0.310,
                        a: 1.0,
                    }),
                ),
                width: 4.0,
                border: iced::Border {
                    radius: Radius::from(10),
                    ..Default::default()
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 6.0 },
                background: Background::Color(Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 1.0,
                }),
                border_width: 0.5,
                border_color: Color::from_rgb(0.3, 0.3, 0.3),
            },
        },
        Status::Hovered => Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color {
                        r: 0.85,
                        g: 0.85,
                        b: 0.85,
                        a: 1.0,
                    }),
                    Background::Color(Color {
                        r: 0.310,
                        g: 0.310,
                        b: 0.310,
                        a: 1.0,
                    }),
                ),
                width: 4.0,
                border: iced::Border {
                    radius: Radius::from(10),
                    ..Default::default()
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 6.0 },
                background: Background::Color(Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 1.0,
                }),
                border_width: 0.5,
                border_color: Color::from_rgb(0.3, 0.3, 0.3),
            },
        },
        Status::Dragged => Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color {
                        r: 0.85,
                        g: 0.85,
                        b: 0.85,
                        a: 1.0,
                    }),
                    Background::Color(Color {
                        r: 0.310,
                        g: 0.310,
                        b: 0.310,
                        a: 1.0,
                    }),
                ),
                width: 4.0,
                border: iced::Border {
                    radius: Radius::from(10),
                    ..Default::default()
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 8.0 },
                background: Background::Color(Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 1.0,
                }),
                border_width: 0.5,
                border_color: Color::from_rgb(0.3, 0.3, 0.3),
            },
        },
    }
}
