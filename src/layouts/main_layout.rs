// External.
use druid::widget::{prelude::*, Button};
use druid::widget::{Flex, Label, MainAxisAlignment, Padding};
use native_dialog::FileDialog;
use swf::{FileAttributes, Tag};

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
                .with_flex_child(
                    Button::from_label(Label::new("Select .swf").with_text_size(TEXT_SIZE))
                        .on_click(MainLayout::on_select_swf_clicked),
                    1.0,
                ),
        )
    }

    fn on_select_swf_clicked(_ctx: &mut EventCtx, _data: &mut ApplicationState, _env: &Env) {
        // Get path to .swf file.
        let path = FileDialog::new()
            .add_filter("Flash Movie", &["swf"])
            .show_open_single_file()
            .unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Parse SWF.
        let sfw_data = std::fs::read(path).unwrap();
        let stream = swf::decompress_swf(&sfw_data[..]).unwrap();
        let swf = swf::parse_swf(&stream).unwrap();

        // Get info.
        let mut _uses_actionscript_3 = false;
        for tag in swf.tags.iter() {
            match tag {
                Tag::FileAttributes(attributes) => {
                    _uses_actionscript_3 = attributes.contains(FileAttributes::IS_ACTION_SCRIPT_3);
                }
                _ => {}
            }
        }

        if _uses_actionscript_3 {
            // use scaleform 4
        } else {
            // use old scaleform
        }
    }
}

impl Default for MainLayout {
    fn default() -> Self {
        Self {}
    }
}
