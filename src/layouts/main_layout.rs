// External.
use druid::widget::prelude::*;
use druid::widget::{Flex, Label, MainAxisAlignment, Padding};

// Custom.
use crate::ApplicationState;

// Layout customization.
const TEXT_SIZE: f64 = 18.0;

#[derive(Clone, Data)]
pub struct MainLayout {}

impl MainLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_ui() -> impl Widget<ApplicationState> {
        Padding::new(
            5.0,
            Flex::column()
                .main_axis_alignment(MainAxisAlignment::Start)
                .must_fill_main_axis(true)
                .with_flex_child(Label::new("Hello World!").with_text_size(TEXT_SIZE), 1.0),
        )
    }
}

impl Default for MainLayout {
    fn default() -> Self {
        Self {}
    }
}
