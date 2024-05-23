use iced::widget::button;
use iced_core::border::Radius;
use iced_core::{Color, Vector, Background, Border, Shadow};

pub struct JustText;

impl button::StyleSheet for JustText {
    type Style = iced::Theme;

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        button::Appearance { shadow_offset: Vector {
            x: 2.0,
            y: 2.0,
        }, background: Some(
                Background::Color(Color
                {
                        r: 0.55,
                        g: 0.55,
                        b: 0.55,
                        a: 1.0
                    }
                )
            ), text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0
            }, border: 
            Border {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0  // no border..
                },
                radius: Radius::default(),
                width: 1.0,
            }
            , shadow: Shadow::default()}
    }

    
    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        button::Appearance { shadow_offset: Vector {
            x: 2.0,
            y: 2.0,
        }, background: None, text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0
            }, border: 
            Border {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0
                },
                radius: Radius::default(),
                width: 1.0,
            }
            , shadow: Shadow::default()}
    }
}
