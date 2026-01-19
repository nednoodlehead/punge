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
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    Background::Color(Color {
                        r: 0.210,
                        g: 0.210,
                        b: 0.210,
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
                border_width: 2.0,
                border_color: Color::BLACK,
            },
        },
        Status::Hovered => Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    Background::Color(Color {
                        r: 0.210,
                        g: 0.210,
                        b: 0.210,
                        a: 1.0,
                    }),
                ),

                width: 4.0,
                border: iced::Border {
                    color: Color::BLACK,
                    width: 1.0,
                    radius: Radius::from(10),
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 6.0 },
                background: Background::Color(Color {
                    r: 0.175,
                    g: 0.175,
                    b: 0.175,
                    a: 1.0,
                }),
                border_width: 10.0,
                border_color: Color::BLACK,
            },
        },
        Status::Dragged => Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(Color::BLACK),
                    Background::Color(Color {
                        r: 0.210,
                        g: 0.210,
                        b: 0.210,
                        a: 1.0,
                    }),
                ),
                width: 4.0,
                border: iced::Border {
                    color: Color::BLACK,
                    width: 1.0,
                    radius: Radius::from(10),
                },
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 4.0 },
                background: Background::Color(Color::BLACK),
                border_width: 0.0,
                border_color: Color::BLACK,
            },
        },
    }
}
