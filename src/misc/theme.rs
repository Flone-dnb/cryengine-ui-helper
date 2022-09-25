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
    pub dangerous: Color,
    pub alt: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct BrightColors {
    pub primary: Color,
    pub secondary: Color,
    pub surface: Color,
    pub dangerous: Color,
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
                    background: Color::from_rgb8(20, 20, 20),
                    foreground: Color::from_rgb8(45, 45, 45),
                },
                normal: NormalColors {
                    primary: Color::from_rgb8(147, 89, 26),
                    secondary: Color::from_rgb8(152, 51, 247),
                    alt: Color::from_rgb8(21, 225, 157),
                    dangerous: Color::from_rgb8(199, 53, 24),
                    surface: Color::from_rgb8(130, 130, 130),
                },
                bright: BrightColors {
                    primary: Color::from_rgb8(244, 155, 53),
                    secondary: Color::from_rgb8(170, 121, 216),
                    alt: Color::from_rgb8(41, 245, 177),
                    dangerous: Color::from_rgb8(219, 73, 44),
                    surface: Color::from_rgb8(224, 224, 224),
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
