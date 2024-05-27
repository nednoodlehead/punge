use iced::widget::container;
use iced_core::border::Radius;
use iced_core::{Background, Border, Color, Shadow, Vector};

pub struct ContainerWithBorder;

impl container::StyleSheet for ContainerWithBorder {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        container::Appearance { text_color: None, background: None, border: Border {
            color: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0
            },
            width: 3.0,
            radius: Radius::from(10.0)
        }, shadow: Shadow {
                color: Color::BLACK,
                offset: Vector { x: 1.0, y: 1.0 },
                blur_radius: 1.0
            } }
    }
}
pub struct BottomBarContainer;

impl container::StyleSheet for BottomBarContainer{
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {

        container::Appearance { text_color: None, background: Some(Background::Color(Color{
            r: 0.1,
            g: 0.1,
            b: 0.1,
            a: 1.0,
        }
        )
        )
            , border: Border {
            color: Color {
                r: 0.3,
                g: 0.3,
                b: 0.3,
                a: 0.0
            },
            width: 0.0,
            radius: Radius::from(0.0)
        }, shadow: Shadow {
                color: Color::BLACK,
                offset: Vector { x: 1.0, y: 1.0 },
                blur_radius: 1.0
            } }

    }
}
