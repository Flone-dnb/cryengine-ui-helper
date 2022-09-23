// External.
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Checkbox, Column, PickList, Row, Scrollable, Text, TextInput},
    Command, Element, Length,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};

// Custom.
use crate::{misc::config::ApplicationConfig, ApplicationMessage};

// Layout customization.
const TEXT_SIZE: u16 = 20;
const SMALL_TEXT_SIZE: u16 = 18;
const ELEMENT_SPACING: u16 = 10;
const TEXT_INPUT_PADDING: u16 = 4;
const PATH_SECTION_LEFT_SIZE_PORTION: u16 = 3;
const PATH_SECTION_RIGHT_SIZE_PORTION: u16 = 7;
const ELEMENT_NAME_SECTION_LEFT_SIZE_PORTION: u16 = 1;
const ELEMENT_NAME_SECTION_RIGHT_SIZE_PORTION: u16 = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VAlign {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

impl std::fmt::Display for VAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VAlign::Top => "Top",
                VAlign::Center => "Center",
                VAlign::Bottom => "Bottom",
            }
        )
    }
}

impl std::fmt::Display for HAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HAlign::Left => "Left",
                HAlign::Center => "Center",
                HAlign::Right => "Right",
            }
        )
    }
}

impl HAlign {
    const ALL: [HAlign; 3] = [HAlign::Left, HAlign::Center, HAlign::Right];
}

impl VAlign {
    const ALL: [VAlign; 3] = [VAlign::Top, VAlign::Center, VAlign::Bottom];
}

enum EntityList {
    Functions,
    Events,
}

#[derive(Debug, Clone)]
pub enum MainLayoutMessage {
    SelectPathToGfxExportBin,
    SelectPathToSwfFile,
    SelectPathToGfxOutput,
    SelectPathToXmlOutput,
    ShowFunctions,
    ShowEvents,
    EntityListAddClicked,
    GenerateClicked,
    AdditionalGfxExportArgsChanged(String),
    UiElementsTextChanged(String),
    UiElementTextChanged(String),
    EntityListItemChanged(usize, String),
    EntityListRemoveItem(String),
    HorizontalAlignChanged(HAlign),
    VerticalAlignChanged(VAlign),
    FullscreenChanged(bool),
}

pub struct MainLayout {
    path_to_gfxexport_bin: String,
    path_to_swf_file: String,
    path_to_gfx_dir: String,
    path_to_xml_dir: String,
    additional_gfxexport_args: String,
    ui_elements_name: String,
    ui_element_name: String,
    current_list: EntityList,
    functions: Vec<String>,
    events: Vec<String>,
    halign: Option<HAlign>,
    valign: Option<VAlign>,
    fullscreen: bool,
}

impl MainLayout {
    pub fn new(app_config: &ApplicationConfig) -> Self {
        Self {
            path_to_gfxexport_bin: app_config.path_to_gfxexport_bin.clone(),
            path_to_swf_file: String::new(),
            path_to_gfx_dir: String::new(),
            path_to_xml_dir: String::new(),
            additional_gfxexport_args: app_config.additional_gfxexport_args.clone(),
            ui_elements_name: String::new(),
            ui_element_name: String::new(),
            functions: Vec::new(),
            events: Vec::new(),
            current_list: EntityList::Functions,
            halign: Some(HAlign::Center),
            valign: Some(VAlign::Center),
            fullscreen: false,
        }
    }

    pub fn view(&self) -> Element<MainLayoutMessage> {
        Column::new()
            .push(
                Row::new()
                    .push(
                        Button::new(Text::new("Path to GFxExport").size(TEXT_SIZE))
                            .on_press(MainLayoutMessage::SelectPathToGfxExportBin)
                            .width(Length::FillPortion(PATH_SECTION_LEFT_SIZE_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Text::new(&self.path_to_gfxexport_bin)
                            .size(SMALL_TEXT_SIZE)
                            .width(Length::FillPortion(PATH_SECTION_RIGHT_SIZE_PORTION))
                            .vertical_alignment(Vertical::Center),
                    ),
            )
            .push(
                Row::new()
                    .push(
                        Text::new("Additional GFxExport arguments")
                            .size(TEXT_SIZE)
                            .width(Length::FillPortion(PATH_SECTION_LEFT_SIZE_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        TextInput::new(
                            "",
                            &self.additional_gfxexport_args,
                            MainLayoutMessage::AdditionalGfxExportArgsChanged,
                        )
                        .padding(TEXT_INPUT_PADDING)
                        .size(TEXT_SIZE)
                        .width(Length::FillPortion(PATH_SECTION_RIGHT_SIZE_PORTION)),
                    ),
            )
            .spacing(ELEMENT_SPACING)
            .push(
                Row::new()
                    .push(
                        Button::new(Text::new("Path to *.swf file").size(TEXT_SIZE))
                            .on_press(MainLayoutMessage::SelectPathToSwfFile)
                            .width(Length::FillPortion(PATH_SECTION_LEFT_SIZE_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Text::new(&self.path_to_swf_file)
                            .size(SMALL_TEXT_SIZE)
                            .width(Length::FillPortion(PATH_SECTION_RIGHT_SIZE_PORTION))
                            .vertical_alignment(Vertical::Center),
                    ),
            )
            .spacing(ELEMENT_SPACING)
            .push(
                Row::new()
                    .push(
                        Button::new(Text::new("Output directory for .gfx files").size(TEXT_SIZE))
                            .on_press(MainLayoutMessage::SelectPathToGfxOutput)
                            .width(Length::FillPortion(PATH_SECTION_LEFT_SIZE_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Text::new(&self.path_to_gfx_dir)
                            .size(SMALL_TEXT_SIZE)
                            .width(Length::FillPortion(PATH_SECTION_RIGHT_SIZE_PORTION))
                            .vertical_alignment(Vertical::Center),
                    ),
            )
            .spacing(ELEMENT_SPACING)
            .push(
                Row::new()
                    .push(
                        Button::new(Text::new("Output directory for .xml files").size(TEXT_SIZE))
                            .on_press(MainLayoutMessage::SelectPathToXmlOutput)
                            .width(Length::FillPortion(PATH_SECTION_LEFT_SIZE_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Text::new(&self.path_to_xml_dir)
                            .size(SMALL_TEXT_SIZE)
                            .width(Length::FillPortion(PATH_SECTION_RIGHT_SIZE_PORTION))
                            .vertical_alignment(Vertical::Center),
                    ),
            )
            .spacing(ELEMENT_SPACING)
            .push(
                Row::new()
                    .push(
                        Text::new("Elements name")
                            .size(TEXT_SIZE)
                            .width(Length::FillPortion(ELEMENT_NAME_SECTION_LEFT_SIZE_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        TextInput::new(
                            "",
                            &self.ui_elements_name,
                            MainLayoutMessage::UiElementsTextChanged,
                        )
                        .padding(TEXT_INPUT_PADDING)
                        .size(TEXT_SIZE)
                        .width(Length::FillPortion(ELEMENT_NAME_SECTION_RIGHT_SIZE_PORTION)),
                    ),
            )
            .spacing(ELEMENT_SPACING)
            .push(
                Row::new()
                    .push(
                        Text::new("Element name")
                            .size(TEXT_SIZE)
                            .width(Length::FillPortion(ELEMENT_NAME_SECTION_LEFT_SIZE_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        TextInput::new(
                            "",
                            &self.ui_element_name,
                            MainLayoutMessage::UiElementTextChanged,
                        )
                        .padding(TEXT_INPUT_PADDING)
                        .size(TEXT_SIZE)
                        .width(Length::FillPortion(ELEMENT_NAME_SECTION_RIGHT_SIZE_PORTION)),
                    ),
            )
            .spacing(ELEMENT_SPACING)
            .push(
                Row::new()
                    .push(
                        Text::new("Horizontal alignment:")
                            .size(TEXT_SIZE)
                            .vertical_alignment(Vertical::Center),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(PickList::new(
                        &HAlign::ALL[..],
                        self.halign,
                        MainLayoutMessage::HorizontalAlignChanged,
                    ))
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Text::new("Vertical alignment:")
                            .size(TEXT_SIZE)
                            .vertical_alignment(Vertical::Center),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(PickList::new(
                        &VAlign::ALL[..],
                        self.valign,
                        MainLayoutMessage::VerticalAlignChanged,
                    ))
                    .spacing(ELEMENT_SPACING)
                    .push(Checkbox::new(
                        self.fullscreen,
                        "Fullscreen",
                        MainLayoutMessage::FullscreenChanged,
                    )),
            )
            .spacing(ELEMENT_SPACING)
            .push(self.get_entity_list())
            .spacing(ELEMENT_SPACING)
            .push(
                Button::new(Text::new("Generate .gfx and .xml files").size(TEXT_SIZE))
                    .on_press(MainLayoutMessage::GenerateClicked)
                    .width(Length::Fill),
            )
            .padding(10)
            .into()
    }

    pub fn update(
        &mut self,
        message: MainLayoutMessage,
        app_config: &mut ApplicationConfig,
    ) -> Command<ApplicationMessage> {
        match message {
            MainLayoutMessage::SelectPathToGfxExportBin => self.select_gfx_bin_path(app_config),
            MainLayoutMessage::SelectPathToSwfFile => self.select_swf_file_path(),
            MainLayoutMessage::SelectPathToGfxOutput => self.select_gfx_output_path(),
            MainLayoutMessage::SelectPathToXmlOutput => self.select_xml_output_path(),
            MainLayoutMessage::UiElementsTextChanged(elements_name) => {
                self.update_ui_elements_name(elements_name)
            }
            MainLayoutMessage::UiElementTextChanged(element_name) => {
                self.update_ui_element_name(element_name)
            }
            MainLayoutMessage::VerticalAlignChanged(valign) => self.update_vertical_align(valign),
            MainLayoutMessage::HorizontalAlignChanged(halign) => {
                self.update_horizontal_align(halign)
            }
            MainLayoutMessage::FullscreenChanged(fullscreen) => self.update_fullscreen(fullscreen),
            MainLayoutMessage::ShowFunctions => self.show_functions(),
            MainLayoutMessage::ShowEvents => self.show_events(),
            MainLayoutMessage::EntityListAddClicked => self.add_list_item(),
            MainLayoutMessage::EntityListItemChanged(index, newname) => {
                self.update_list_item(index, newname)
            }
            MainLayoutMessage::EntityListRemoveItem(name) => self.remove_list_item(name),
            MainLayoutMessage::AdditionalGfxExportArgsChanged(args) => {
                self.update_additional_gfxexport_args(args)
            }
            MainLayoutMessage::GenerateClicked => self.generate(app_config),
        }

        Command::none()
    }

    fn get_entity_list(&self) -> Element<MainLayoutMessage> {
        let mut list = Column::new();

        // Prepare buttons for categories.
        let mut functions_button = Button::new(Text::new("Functions").size(TEXT_SIZE))
            .on_press(MainLayoutMessage::ShowFunctions)
            .style(iced::theme::Button::Secondary)
            .width(Length::Fill);
        let mut events_button = Button::new(Text::new("Events").size(TEXT_SIZE))
            .on_press(MainLayoutMessage::ShowEvents)
            .style(iced::theme::Button::Secondary)
            .width(Length::Fill);

        // Highlight active.
        match self.current_list {
            EntityList::Functions => {
                functions_button = functions_button.style(iced::theme::Button::Primary)
            }
            EntityList::Events => events_button = events_button.style(iced::theme::Button::Primary),
        }

        // Add buttons to switch lists.
        list = list.push(
            Row::new()
                .push(functions_button)
                .spacing(ELEMENT_SPACING)
                .push(events_button),
        );
        list = list.spacing(ELEMENT_SPACING);

        // Get reference to current array.
        let mut _vec_to_use = &self.functions;
        match self.current_list {
            EntityList::Functions => {
                _vec_to_use = &self.functions;
            }
            EntityList::Events => {
                _vec_to_use = &self.events;
            }
        }

        // Fill list.
        for (index, item) in _vec_to_use.iter().enumerate() {
            list = list.push(
                Row::new()
                    .push(
                        TextInput::new("name", &item, move |name: String| -> MainLayoutMessage {
                            MainLayoutMessage::EntityListItemChanged(index, name)
                        })
                        .size(TEXT_SIZE)
                        .padding(TEXT_INPUT_PADDING),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Button::new(Text::new("Remove").size(TEXT_SIZE))
                            .on_press(MainLayoutMessage::EntityListRemoveItem(item.clone())),
                    ),
            );
        }

        // Add "Add" button to list.
        list = list.push(
            Button::new(
                Text::new("Add")
                    .size(TEXT_SIZE)
                    .horizontal_alignment(Horizontal::Center),
            )
            .on_press(MainLayoutMessage::EntityListAddClicked)
            .width(Length::Fill),
        );

        Scrollable::new(list).height(Length::Fill).into()
    }

    fn generate(&mut self, app_config: &mut ApplicationConfig) {
        // Save additional GFxExport arguments to config.
        app_config.additional_gfxexport_args = self.additional_gfxexport_args.clone();
        if let Err(app_error) = app_config.save() {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(&format!(
                    "Failed to save configuration file to {}.\n\nError: {}",
                    ApplicationConfig::get_config_file_path().to_string_lossy(),
                    app_error
                ))
                .show_alert()
                .unwrap();
        }

        // TODO

        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Info")
            .set_text("Successfully generated .gfx and .xml files.")
            .show_alert()
            .unwrap();
    }

    fn update_additional_gfxexport_args(&mut self, args: String) {
        self.additional_gfxexport_args = args;
    }

    fn update_list_item(&mut self, index: usize, newname: String) {
        // Get reference to current array.
        let mut _vec_to_use = &mut self.functions;
        match self.current_list {
            EntityList::Functions => {
                _vec_to_use = &mut self.functions;
            }
            EntityList::Events => {
                _vec_to_use = &mut self.events;
            }
        }

        _vec_to_use[index] = newname;
    }

    fn add_list_item(&mut self) {
        // Get reference to current array.
        let mut _vec_to_use = &mut self.functions;
        match self.current_list {
            EntityList::Functions => {
                _vec_to_use = &mut self.functions;
            }
            EntityList::Events => {
                _vec_to_use = &mut self.events;
            }
        }

        _vec_to_use.push(String::from("name"));
    }

    fn remove_list_item(&mut self, name: String) {
        // Get reference to current array.
        let mut _vec_to_use = &mut self.functions;
        match self.current_list {
            EntityList::Functions => {
                _vec_to_use = &mut self.functions;
            }
            EntityList::Events => {
                _vec_to_use = &mut self.events;
            }
        }

        // Remove item.
        for (i, item) in _vec_to_use.iter().enumerate() {
            if item == &name {
                _vec_to_use.remove(i);
                break;
            }
        }
    }

    fn show_functions(&mut self) {
        self.current_list = EntityList::Functions;
    }

    fn show_events(&mut self) {
        self.current_list = EntityList::Events;
    }

    fn update_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
    }

    fn update_horizontal_align(&mut self, halign: HAlign) {
        self.halign = Some(halign);
    }

    fn update_vertical_align(&mut self, valign: VAlign) {
        self.valign = Some(valign);
    }

    fn update_ui_elements_name(&mut self, elements_name: String) {
        self.ui_elements_name = elements_name;
    }

    fn update_ui_element_name(&mut self, element_name: String) {
        self.ui_element_name = element_name;
    }

    fn select_xml_output_path(&mut self) {
        // Get path to output .xml files.
        let path = FileDialog::new().show_open_single_dir().unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Save.
        self.path_to_xml_dir = path.to_string_lossy().to_string();
    }

    fn select_gfx_output_path(&mut self) {
        // Get path to output .gfx files.
        let path = FileDialog::new().show_open_single_dir().unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Save.
        self.path_to_gfx_dir = path.to_string_lossy().to_string();
    }

    fn select_swf_file_path(&mut self) {
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
        self.path_to_swf_file = path.to_string_lossy().to_string();

        // Save paths to output directies.
        if path.parent().is_some() && path.parent().unwrap().parent().is_some() {
            // Set path to .gfx and .xml files.
            let path_to_gfx = path.parent().unwrap().parent().unwrap();
            let mut path_to_xml = path_to_gfx.to_path_buf();
            path_to_xml.push("UIElements");

            self.path_to_gfx_dir = path_to_gfx.to_string_lossy().to_string();
            self.path_to_xml_dir = path_to_xml.to_string_lossy().to_string();
        }

        // Set UI elemnt names.
        self.ui_elements_name = path.file_stem().unwrap().to_string_lossy().to_string();
        self.ui_element_name = self.ui_elements_name.clone();
    }

    fn select_gfx_bin_path(&mut self, app_config: &mut ApplicationConfig) {
        // Get path to GFxExport1.exe file.
        let path = FileDialog::new()
            .add_filter("GFxExport file", &["exe"])
            .show_open_single_file()
            .unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Save to UI.
        self.path_to_gfxexport_bin = path.to_string_lossy().to_string();

        // Save to config.
        app_config.path_to_gfxexport_bin = self.path_to_gfxexport_bin.clone();
        if let Err(app_error) = app_config.save() {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(&format!(
                    "Failed to save configuration file to {}.\n\nError: {}",
                    ApplicationConfig::get_config_file_path().to_string_lossy(),
                    app_error
                ))
                .show_alert()
                .unwrap();
        }
    }
}
