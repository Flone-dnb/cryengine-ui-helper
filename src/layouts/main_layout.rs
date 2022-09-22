use druid::{Lens, LensExt, WidgetExt};
// External.
use druid::widget::{prelude::*, Button, Checkbox, RadioGroup, TextBox};
use druid::widget::{Flex, Label, MainAxisAlignment, Padding};
use native_dialog::FileDialog;

// Custom.
use crate::ApplicationState;

// Layout customization.
const TEXT_SIZE: f64 = 15.0;
const SMALL_TEXT_SIZE: f64 = 13.0;
const LEFT_SIDE_SIZE: f64 = 0.35;
const RIGHT_SIDE_SIZE: f64 = 0.65;
const UI_ELEMENT_LEFT_SIDE_SIZE: f64 = 0.2;
const UI_ELEMENT_RIGHT_SIDE_SIZE: f64 = 0.8;
const PATH_BOX_HEIGHT: f64 = 0.04;
const TEXT_BOX_HEIGHT: f64 = 0.02;
const CONSTRAINS_BOX_HEIGHT: f64 = 0.15;

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Data, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Data, Lens)]
pub struct MainLayout {
    pub path_to_gfxexport_bin: String,
    pub path_to_swf_file: String,
    pub path_to_gfx_dir: String,
    pub path_to_xml_dir: String,
    pub ui_elements_name: String,
    pub ui_element_name: String,
    pub is_fullscreen: bool,
    pub halign: HAlign,
    pub valign: VAlign,
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
                                Label::new("Path to GFxExport").with_text_size(TEXT_SIZE),
                            )
                            .expand_width()
                            .on_click(MainLayout::on_select_gfxexport_bin_clicked),
                            LEFT_SIDE_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_gfxexport_bin.clone()
                            })
                            .with_text_size(SMALL_TEXT_SIZE),
                            RIGHT_SIDE_SIZE,
                        ),
                    PATH_BOX_HEIGHT,
                )
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Path to *.swf file").with_text_size(TEXT_SIZE),
                            )
                            .expand_width()
                            .on_click(MainLayout::on_select_swf_clicked),
                            LEFT_SIDE_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_swf_file.clone()
                            })
                            .with_text_size(SMALL_TEXT_SIZE),
                            RIGHT_SIDE_SIZE,
                        ),
                    PATH_BOX_HEIGHT,
                )
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Output directory for .gfx files")
                                    .with_text_size(TEXT_SIZE),
                            )
                            .expand_width()
                            .on_click(MainLayout::on_select_gfx_clicked),
                            LEFT_SIDE_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_gfx_dir.clone()
                            })
                            .with_text_size(SMALL_TEXT_SIZE),
                            RIGHT_SIDE_SIZE,
                        ),
                    PATH_BOX_HEIGHT,
                )
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Button::from_label(
                                Label::new("Output directory for .xml files")
                                    .with_text_size(TEXT_SIZE),
                            )
                            .expand_width()
                            .on_click(MainLayout::on_select_xml_clicked),
                            LEFT_SIDE_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Label::new(|data: &ApplicationState, _env: &_| {
                                data.main_layout.path_to_xml_dir.clone()
                            })
                            .with_text_size(SMALL_TEXT_SIZE),
                            RIGHT_SIDE_SIZE,
                        ),
                    PATH_BOX_HEIGHT,
                )
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Label::new("Elements name")
                                .with_text_size(TEXT_SIZE)
                                .expand_width(),
                            UI_ELEMENT_LEFT_SIDE_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            TextBox::new()
                                .with_text_size(TEXT_SIZE)
                                .lens(
                                    ApplicationState::main_layout
                                        .then(MainLayout::ui_elements_name),
                                )
                                .expand(),
                            UI_ELEMENT_RIGHT_SIDE_SIZE,
                        ),
                    TEXT_BOX_HEIGHT,
                )
                .with_default_spacer()
                .with_flex_child(
                    Flex::row()
                        .with_flex_child(
                            Label::new("Element name")
                                .with_text_size(TEXT_SIZE)
                                .expand_width(),
                            UI_ELEMENT_LEFT_SIDE_SIZE,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            TextBox::new()
                                .with_text_size(TEXT_SIZE)
                                .lens(
                                    ApplicationState::main_layout.then(MainLayout::ui_element_name),
                                )
                                .expand(),
                            UI_ELEMENT_RIGHT_SIDE_SIZE,
                        ),
                    TEXT_BOX_HEIGHT,
                )
                .with_flex_child(
                    Checkbox::from_label(Label::new("Fullscreen").with_text_size(TEXT_SIZE))
                        .align_left()
                        .lens(ApplicationState::main_layout.then(MainLayout::is_fullscreen)),
                    PATH_BOX_HEIGHT,
                )
                .with_flex_child(
                    Flex::row()
                        .must_fill_main_axis(true)
                        .with_flex_child(
                            Flex::column()
                                .with_flex_child(
                                    Label::new("Horizontal alignment:").with_text_size(TEXT_SIZE),
                                    1.0,
                                )
                                .with_default_spacer()
                                .with_flex_child(
                                    RadioGroup::new(vec![
                                        ("left", HAlign::Left),
                                        ("center", HAlign::Center),
                                        ("right", HAlign::Right),
                                    ])
                                    .lens(ApplicationState::main_layout.then(MainLayout::halign)),
                                    1.0,
                                ),
                            0.5,
                        )
                        .with_default_spacer()
                        .with_flex_child(
                            Flex::column()
                                .with_flex_child(
                                    Label::new("Vertical alignment:").with_text_size(TEXT_SIZE),
                                    1.0,
                                )
                                .with_default_spacer()
                                .with_flex_child(
                                    RadioGroup::new(vec![
                                        ("top", VAlign::Top),
                                        ("center", VAlign::Center),
                                        ("bottom", VAlign::Bottom),
                                    ])
                                    .lens(ApplicationState::main_layout.then(MainLayout::valign)),
                                    1.0,
                                ),
                            0.5,
                        ),
                    CONSTRAINS_BOX_HEIGHT,
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

        // Save paths to output directies.
        if path.parent().is_some() && path.parent().unwrap().parent().is_some() {
            // Set path to .gfx and .xml files.
            let path_to_gfx = path.parent().unwrap().parent().unwrap();
            let mut path_to_xml = path_to_gfx.to_path_buf();
            path_to_xml.push("UIElements");

            data.main_layout.path_to_gfx_dir = path_to_gfx.to_string_lossy().to_string();
            data.main_layout.path_to_xml_dir = path_to_xml.to_string_lossy().to_string();
        }

        // Set UI elemnt names.
        data.main_layout.ui_elements_name = path.file_stem().unwrap().to_string_lossy().to_string();
        data.main_layout.ui_element_name = data.main_layout.ui_elements_name.clone();
    }

    fn on_select_xml_clicked(_ctx: &mut EventCtx, data: &mut ApplicationState, _env: &Env) {
        // Get path to output .xml files.
        let path = FileDialog::new().show_open_single_dir().unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Save.
        data.main_layout.path_to_xml_dir = path.to_string_lossy().to_string();
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
            ui_elements_name: String::new(),
            ui_element_name: String::new(),
            is_fullscreen: false,
            halign: HAlign::Center,
            valign: VAlign::Center,
        }
    }
}
