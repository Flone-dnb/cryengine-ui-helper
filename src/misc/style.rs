// External.
use iced::overlay::menu;
use iced::widget::{
    button, checkbox, container, pick_list, radio, rule, scrollable, text, text_input,
};
use iced::{application, Background, Color};

// Custom.
use super::theme::Theme;

#[derive(Default, Debug, Clone, Copy)]
pub enum Application {
    #[default]
    Default,
}

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: self.palette().base.background,
            text_color: self.palette().bright.surface,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Container {
    #[default]
    Invisible,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: Self::Style) -> container::Appearance {
        match style {
            Container::Invisible => container::Appearance::default(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    #[default]
    Default,
    Inactive,
    Special,
    Dangerous,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: Self::Style) -> button::Appearance {
        let p = self.palette();

        let appearance = button::Appearance {
            border_width: 1.0,
            border_radius: 2.0,
            ..button::Appearance::default()
        };

        let active_appearance = |bg: Option<Color>, mc| button::Appearance {
            background: Some(Background::Color(bg.unwrap_or(p.base.foreground))),
            border_color: Color { a: 0.5, ..mc },
            text_color: mc,
            ..appearance
        };

        match style {
            Button::Default => active_appearance(None, p.bright.primary),
            Button::Inactive => active_appearance(None, p.normal.primary),
            Button::Special => active_appearance(None, p.bright.alt),
            Button::Dangerous => active_appearance(None, p.bright.dangerous),
        }
    }

    fn hovered(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);
        let p = self.palette();

        let hover_appearance = |bg, tc: Option<Color>| button::Appearance {
            background: Some(Background::Color(Color { a: 0.25, ..bg })),
            text_color: tc.unwrap_or(bg),
            ..active
        };

        match style {
            Button::Default => hover_appearance(p.bright.primary, None),
            Button::Inactive => hover_appearance(p.normal.primary, None),
            Button::Special => hover_appearance(p.bright.alt, None),
            Button::Dangerous => hover_appearance(p.bright.dangerous, None),
        }
    }

    fn disabled(&self, style: Self::Style) -> button::Appearance {
        let active = self.active(style);

        match style {
            _ => button::Appearance { ..active },
        }
    }

    fn pressed(&self, style: Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum Scrollable {
    #[default]
    Default,
}

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, style: Self::Style) -> scrollable::Scrollbar {
        let from_appearance = |c: Color| scrollable::Scrollbar {
            background: Some(Background::Color(c)),
            border_radius: 5.0,
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            scroller: scrollable::Scroller {
                color: self.palette().normal.primary,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
        };

        match style {
            Scrollable::Default => from_appearance(self.palette().base.foreground),
        }
    }

    fn hovered(&self, style: Self::Style) -> scrollable::Scrollbar {
        scrollable::Scrollbar {
            scroller: scrollable::Scroller {
                ..self.active(style).scroller
            },
            ..self.active(style)
        }
    }

    fn dragging(&self, style: Self::Style) -> scrollable::Scrollbar {
        let hovered = self.hovered(style);
        scrollable::Scrollbar {
            scroller: scrollable::Scroller { ..hovered.scroller },
            ..hovered
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum CheckBox {
    #[default]
    Default,
}

impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, style: Self::Style, _is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckBox::Default => checkbox::Appearance {
                background: Background::Color(self.palette().base.background),
                checkmark_color: self.palette().bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette().normal.primary,
                text_color: Some(self.palette().bright.surface),
            },
        }
    }

    fn hovered(&self, style: Self::Style, _is_checked: bool) -> checkbox::Appearance {
        match style {
            CheckBox::Default => checkbox::Appearance {
                background: Background::Color(Color {
                    a: 0.25,
                    ..self.palette().normal.primary
                }),
                checkmark_color: self.palette().bright.primary,
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette().normal.primary,
                text_color: Some(self.palette().bright.surface),
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum TextInput {
    #[default]
    Default,
    Special,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                background: Background::Color(self.palette().base.foreground),
                border_radius: 5.0,
                border_width: 0.0,
                border_color: self.palette().base.foreground,
            },
            TextInput::Special => text_input::Appearance {
                background: Background::Color(self.palette().base.foreground),
                border_radius: 5.0,
                border_width: 1.0,
                border_color: self.palette().normal.secondary,
            },
        }
    }

    fn focused(&self, style: Self::Style) -> text_input::Appearance {
        match style {
            TextInput::Default => text_input::Appearance {
                background: Background::Color(self.palette().base.foreground),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..self.palette().normal.primary
                },
            },
            TextInput::Special => text_input::Appearance {
                background: Background::Color(self.palette().base.foreground),
                border_radius: 2.0,
                border_width: 1.0,
                border_color: Color {
                    a: 0.5,
                    ..self.palette().bright.secondary
                },
            },
        }
    }

    fn placeholder_color(&self, _style: Self::Style) -> Color {
        self.palette().normal.surface
    }

    fn value_color(&self, _style: Self::Style) -> Color {
        self.palette().bright.primary
    }

    fn selection_color(&self, _style: Self::Style) -> Color {
        self.palette().normal.primary
    }

    /// Produces the style of an hovered text input.
    fn hovered(&self, style: Self::Style) -> text_input::Appearance {
        self.focused(style)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum PickList {
    #[default]
    Default,
}

impl menu::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> menu::Appearance {
        let p = self.palette();

        menu::Appearance {
            text_color: p.bright.surface,
            background: p.base.background.into(),
            border_width: 1.0,
            border_radius: 2.0,
            border_color: p.base.background,
            selected_text_color: p.bright.surface,
            selected_background: p.normal.primary.into(),
        }
    }
}

impl pick_list::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: ()) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: self.palette().bright.surface,
            background: self.palette().base.background.into(),
            border_width: 1.0,
            border_color: Color {
                a: 0.5,
                ..self.palette().normal.primary
            },
            border_radius: 2.0,
            icon_size: 0.5,
            placeholder_color: self.palette().bright.surface,
        }
    }

    fn hovered(&self, style: ()) -> pick_list::Appearance {
        let active = self.active(style);
        pick_list::Appearance {
            border_color: self.palette().normal.primary,
            ..active
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Text {
    #[default]
    Default,
    Color(Color),
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::Color(color)
    }
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        match style {
            Text::Default => Default::default(),
            Text::Color(c) => text::Appearance { color: Some(c) },
        }
    }
}

impl radio::StyleSheet for Theme {
    type Style = ();

    fn active(&self, _style: Self::Style) -> radio::Appearance {
        radio::Appearance {
            background: Color::TRANSPARENT.into(),
            dot_color: self.palette().bright.primary,
            border_width: 1.0,
            border_color: self.palette().bright.primary,
            text_color: None,
        }
    }

    fn hovered(&self, style: Self::Style) -> radio::Appearance {
        let active = self.active(style);

        radio::Appearance {
            dot_color: self.palette().bright.primary,
            border_color: self.palette().bright.primary,
            border_width: 2.0,
            ..active
        }
    }
}

#[derive(Default, Clone, Copy)]
pub enum Rule {
    #[default]
    Default,
}

impl rule::StyleSheet for Theme {
    type Style = Rule;

    fn style(&self, style: Self::Style) -> rule::Appearance {
        match style {
            Rule::Default => rule::Appearance {
                color: self.palette().bright.surface,
                width: 2,
                radius: 2.0,
                fill_mode: rule::FillMode::Full,
            },
        }
    }
}
