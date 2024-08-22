use iced::border::Radius;
use iced::widget::slider;
use iced::widget::slider::{Status, Style};
use iced::Color;

pub fn volume_style(status: Status) -> Style {
    match status {
        Status::Active => Style {
            rail: slider::Rail {
                colors: (
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    Color {
                        r: 0.210,
                        g: 0.210,
                        b: 0.210,
                        a: 1.0,
                    },
                ),
                width: 4.0,
                border_radius: Radius::from(10),
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 6.0 },
                color: Color {
                    r: 0.175,
                    g: 0.175,
                    b: 0.175,
                    a: 1.0,
                },
                border_width: 10.0,
                border_color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        },
        Status::Hovered => Style {
            rail: slider::Rail {
                colors: (
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    Color {
                        r: 0.210,
                        g: 0.210,
                        b: 0.210,
                        a: 1.0,
                    },
                ),
                width: 4.0,
                border_radius: Radius::from(10),
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 6.0 },
                color: Color {
                    r: 0.175,
                    g: 0.175,
                    b: 0.175,
                    a: 1.0,
                },
                border_width: 10.0,
                border_color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
            },
        },
        Status::Dragged => Style {
            rail: slider::Rail {
                colors: (
                    Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    },
                    Color {
                        r: 0.210,
                        g: 0.210,
                        b: 0.210,
                        a: 1.0,
                    },
                ),
                width: 4.0,
                border_radius: Radius::from(10),
            },
            handle: slider::Handle {
                shape: iced::widget::vertical_slider::HandleShape::Circle { radius: 4.0 },
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                },
                border_width: 0.0,
                border_color: Color::BLACK,
            },
        },
    }
}
