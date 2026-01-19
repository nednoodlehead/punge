use iced::border::Radius;
use iced::widget::container::Style;
use iced::{Background, Border, Color, Shadow, Vector};

// most was for debugging the container size lol
pub fn _container_with_border() -> Style {
    Style {
        text_color: None,
        background: None,
        border: Border {
            color: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            },
            width: 3.0,
            radius: Radius::from(10.0),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 1.0, y: 1.0 },
            blur_radius: 1.0,
        },
        snap: false,
    }
}

pub fn bottom_bar_container() -> Style {
    Style {
        text_color: None,
        background: Some(Background::Color(Color {
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        })),
        border: Border {
            color: Color {
                r: 0.3,
                g: 0.3,
                b: 0.3,
                a: 0.0,
            },
            width: 0.0,
            radius: Radius::from(0.0),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector { x: 1.0, y: 1.0 },
            blur_radius: 1.0,
        },
        snap: false,
    }
}
