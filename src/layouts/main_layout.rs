// External.
use druid::widget::{prelude::*, Button};
use druid::widget::{Flex, Label, MainAxisAlignment, Padding};
use native_dialog::FileDialog;

// Custom.
use crate::ApplicationState;

// Layout customization.
const TEXT_SIZE: f64 = 18.0;
const SMALL_TEXT_SIZE: f64 = 14.0;
const LEFT_COLUMN_SIZE: f64 = 0.4;
const RIGHT_COLUMN_SIZE: f64 = 0.6;

#[derive(Clone, Data)]
pub struct MainLayout {
    pub path_to_gfxexport_bin: String,
    pub path_to_swf_file: String,
    pub path_to_gfx_dir: String,
    pub path_to_xml_dir: String,
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
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Path to GFxExport:").with_text_size(TEXT_SIZE),
                            )
                            .on_click(MainLayout::on_select_gfxexport_bin_clicked),
                            LEFT_COLUMN_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_gfxexport_bin.clone()
                            })
                            .with_text_size(SMALL_TEXT_SIZE),
                            RIGHT_COLUMN_SIZE,
                        ),
                    1.0,
                )
                .with_default_spacer()
                .with_default_spacer()
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Path to *.swf file:").with_text_size(TEXT_SIZE),
                            )
                            .on_click(MainLayout::on_select_swf_clicked),
                            LEFT_COLUMN_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_swf_file.clone()
                            })
                            .with_text_size(SMALL_TEXT_SIZE),
                            RIGHT_COLUMN_SIZE,
                        ),
                    1.0,
                )
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Output directory for .gfx files:")
                                    .with_text_size(TEXT_SIZE),
                            )
                            .on_click(MainLayout::on_select_gfx_clicked),
                            LEFT_COLUMN_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_gfx_dir.clone()
                            })
                            .with_text_size(SMALL_TEXT_SIZE),
                            RIGHT_COLUMN_SIZE,
                        ),
                    1.0,
                ),
        )
    }

    fn on_select_gfxexport_bin_clicked(
        _ctx: &mut EventCtx,
        data: &mut ApplicationState,
        _env: &Env,
    ) {
        // Get path to GFxExport1.exe file.
        let path = FileDialog::new()
            .add_filter("GFxExport file", &["exe"])
            .show_open_single_file()
            .unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Save.
        data.main_layout.path_to_gfxexport_bin = path.to_string_lossy().to_string();
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

        // Save.
        data.main_layout.path_to_swf_file = path.to_string_lossy().to_string();

        if data.main_layout.path_to_gfx_dir.is_empty()
            && path.parent().is_some()
            && path.parent().unwrap().parent().is_some()
        {
            // Set path to .gfx files one directory above.
            data.main_layout.path_to_gfx_dir = path
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_string_lossy()
                .to_string();
        }
    }

    fn on_select_gfx_clicked(_ctx: &mut EventCtx, data: &mut ApplicationState, _env: &Env) {
        // Get path to output .gfx files.
        let path = FileDialog::new().show_open_single_dir().unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Save.
        data.main_layout.path_to_gfx_dir = path.to_string_lossy().to_string();
    }
}

impl Default for MainLayout {
    fn default() -> Self {
        Self {
            path_to_swf_file: String::new(),
            path_to_gfxexport_bin: String::new(),
            path_to_gfx_dir: String::new(),
            path_to_xml_dir: String::new(),
        }
    }
}
