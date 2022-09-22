// External.
use druid::widget::{prelude::*, Button};
use druid::widget::{Flex, Label, MainAxisAlignment, Padding};
use native_dialog::{FileDialog, MessageDialog, MessageType};

// Custom.
use crate::ApplicationState;

// Layout customization.
const TEXT_SIZE: f64 = 18.0;

#[derive(Clone, Data)]
pub struct MainLayout {
    pub path_to_swf: String,
    pub path_to_engine: String,
}

impl MainLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_ui() -> impl Widget<ApplicationState> {
        Padding::new(
            10.0,
            Flex::column()
                .main_axis_alignment(MainAxisAlignment::Start)
                .must_fill_main_axis(true)
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Select path to engine").with_text_size(TEXT_SIZE),
                            )
                            .on_click(MainLayout::on_select_engine_clicked),
                            0.3,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_engine.clone()
                            })
                            .with_text_size(TEXT_SIZE),
                            0.7,
                        ),
                    1.0,
                )
                .with_default_spacer()
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Select .swf file").with_text_size(TEXT_SIZE),
                            )
                            .on_click(MainLayout::on_select_swf_clicked),
                            0.3,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_swf.clone()
                            })
                            .with_text_size(TEXT_SIZE),
                            0.7,
                        ),
                    1.0,
                ),
        )
    }

    fn on_select_swf_clicked(_ctx: &mut EventCtx, data: &mut ApplicationState, _env: &Env) {
        // Get path to .swf file.
        let path = FileDialog::new()
            .add_filter("SWF Movie", &["swf"])
            .show_open_single_file()
            .unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();
        data.main_layout.path_to_swf = path.to_string_lossy().to_string();
    }

    fn on_select_engine_clicked(_ctx: &mut EventCtx, data: &mut ApplicationState, _env: &Env) {
        // Get path to engine directory.
        let path = FileDialog::new().show_open_single_dir().unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Check that this directory contains "Tools/GFxExport" directories.
        let mut path_to_tool = path.clone();
        path_to_tool.push("Tools");
        path_to_tool.push("GFxExport");

        if !path_to_tool.exists() {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(
                    "Path to engine should point to engine directory where file \
                    \"cryengine.cryengine\" is located.",
                )
                .show_alert()
                .unwrap();
            Self::on_select_engine_clicked(_ctx, data, _env);
            return;
        }

        data.main_layout.path_to_engine = path.to_string_lossy().to_string();
    }
}

impl Default for MainLayout {
    fn default() -> Self {
        Self {
            path_to_swf: String::new(),
            path_to_engine: String::new(),
        }
    }
}
