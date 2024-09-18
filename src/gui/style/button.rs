use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::{Background, Border, Color, Shadow, Vector};

pub fn just_text(status: Status) -> iced::widget::button::Style {
    match status {
        Status::Active => Style {
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
        },
        Status::Hovered => {
            Style {
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
        _ => Style::default(),
    }
}

pub fn menu_button_style(status: Status) -> Style {
    match status {
        Status::Hovered => Style {
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
        },
        Status::Active => Style {
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
        },

        Status::Disabled => Style {
            background: None,
            text_color: Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            },
            ..Default::default()
        },
        _ => Style::default(),
    }
}

pub fn sub_menu_button_style(status: Status) -> Style {
    match status {
        Status::Active => Style {
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
        },
        Status::Hovered => {
            Style {
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
        _ => Style::default(),
    }
}

pub fn playlist_text_style(status: Status) -> Style {
    match status {
        Status::Disabled => Style {
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
        },

        _ => Style::default(),
    }
}

// defines regular buttons, but styled so they aren't default. persistent GOTO search `confirm` button, `download` button

pub fn punge_button_style(status: Status) -> Style {
    match status {
        Status::Active => Style {
            background: Some(Background::Color(Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            })),
            text_color: Color {
                r: 0.85,
                g: 0.85,
                b: 0.85,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.75,
                    g: 0.75,
                    b: 0.75,
                    a: 1.0,
                },
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        },
        Status::Disabled => Style {
            background: Some(Background::Color(Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            })),
            text_color: Color {
                r: 0.85,
                g: 0.85,
                b: 0.85,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.75,
                    g: 0.75,
                    b: 0.75,
                    a: 1.0,
                },
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        },
        Status::Hovered => Style {
            background: Some(Background::Color(Color {
                r: 0.0,
                g: 0.5,
                b: 0.2,
                a: 1.0,
            })),
            text_color: Color {
                r: 0.85,
                g: 0.85,
                b: 0.85,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.75,
                    g: 0.75,
                    b: 0.75,
                    a: 1.0,
                },
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        },
        Status::Pressed => Style {
            background: Some(Background::Color(Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            })),
            text_color: Color {
                r: 0.3,
                g: 0.3,
                b: 0.3,
                a: 1.0,
            },
            border: Border {
                color: Color {
                    r: 0.75,
                    g: 0.75,
                    b: 0.75,
                    a: 1.0,
                },
                radius: Radius::default(),
                width: 1.0,
            },
            shadow: Shadow::default(),
        },

        _ => Style::default(),
    }
}
