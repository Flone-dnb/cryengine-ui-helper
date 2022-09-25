use iced::Color;

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Theme {
    #[default]
    DarkOrange,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseColors {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub alt: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub alt: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    pub base: BaseColors,
    pub normal: NormalColors,
    pub bright: BrightColors,
}

impl Theme {
    pub fn palette(&self) -> ColorPalette {
        match self {
            Self::DarkOrange => ColorPalette {
                base: BaseColors {
                    background: Color::from_rgb8(40, 40, 40),
                    foreground: Color::from_rgb8(28, 28, 28),
                },
                normal: NormalColors {
                    primary: Color::from_rgb8(147, 89, 26),
                    secondary: Color::from_rgb8(170, 121, 216),
                    surface: Color::from_rgb8(130, 130, 130),
                    alt: Color::from_rgb8(153, 43, 43),
                },
                bright: BrightColors {
                    primary: Color::from_rgb8(244, 155, 53),
                    secondary: Color::from_rgb8(152, 51, 247),
                    surface: Color::from_rgb8(224, 224, 224),
                    alt: Color::from_rgb8(193, 48, 71),
                },
            },
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Theme::DarkOrange => "Dark Orange",
            }
        )
    }
}
