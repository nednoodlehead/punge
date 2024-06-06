use iced::widget::button;
use iced_core::border::Radius;
use iced_core::{Background, Border, Color, Shadow, Vector};

pub struct JustText;

impl button::StyleSheet for JustText {
    type Style = iced::Theme;

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 2.0, y: 2.0 },
            background: None,
            text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0, // no border..
                },
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        }
    }

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 2.0, y: 2.0 },
            background: None,
            text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        }
    }
}

pub struct MenuButton;

impl button::StyleSheet for MenuButton {
    type Style = iced::Theme;

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 2.0, y: 2.0 },
            background: Some(Background::Color(Color {
                r: 0.55,
                g: 0.55,
                b: 0.55,
                a: 1.0,
            })),
            text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.8,
                    g: 0.8,
                    b: 0.8,
                    a: 1.0,
                },
                width: 2.0,
                radius: Radius::from(20.0),
            },
            shadow: Shadow {
                color: Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 0.1,
                },
                offset: Vector { x: 3.0, y: 3.0 },
                blur_radius: 1.0,
            },
        }
    }
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 2.0, y: 2.0 },
            background: None,
            text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.8,
                    g: 0.8,
                    b: 0.8,
                    a: 0.0,
                },
                width: 2.0,
                radius: Radius::from(20.0),
            },
            shadow: Shadow {
                color: Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 0.1,
                },
                offset: Vector { x: 3.0, y: 3.0 },
                blur_radius: 1.0,
            },
        }
    }
}

pub struct SubMenuButton;

impl button::StyleSheet for SubMenuButton {
    type Style = iced::Theme;

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 2.0, y: 2.0 },
            background: Some(Background::Color(Color {
                r: 0.55,
                g: 0.55,
                b: 0.55,
                a: 1.0,
            })),
            text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.8,
                    g: 0.8,
                    b: 0.8,
                    a: 1.0,
                },
                width: 2.0,
                radius: Radius::from(0.0), // 20 -> 0
            },
            shadow: Shadow {
                color: Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 0.1,
                },
                offset: Vector { x: 3.0, y: 3.0 },
                blur_radius: 1.0,
            },
        }
    }
    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 2.0, y: 2.0 },
            background: None,
            text_color: Color {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.8,
                    g: 0.8,
                    b: 0.8,
                    a: 0.0,
                },
                width: 2.0,
                radius: Radius::from(20.0),
            },
            shadow: Shadow {
                color: Color {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 0.1,
                },
                offset: Vector { x: 3.0, y: 3.0 },
                blur_radius: 1.0,
            },
        }
    }
}

pub struct PlaylistText;

impl button::StyleSheet for PlaylistText {
    type Style = iced::Theme;

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: Vector { x: 2.0, y: 2.0 },
            background: None,
            text_color: Color {
                r: 0.85,
                g: 0.85,
                b: 0.85,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        }
    }

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        button::Appearance::default() // never used, required function
    }
}
