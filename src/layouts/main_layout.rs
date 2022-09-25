// External.
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Button, Checkbox, Column, PickList, Row, Scrollable, Text, TextInput},
    Command, Element, Length, Renderer,
};
use native_dialog::{FileDialog, MessageDialog, MessageType};

// STD.
use std::process;
use std::{fs, path::Path};

// Custom.
use crate::{
    managers::xml_manager::*,
    misc::{config::ApplicationConfig, style, theme::Theme},
    ApplicationMessage,
};

// Layout customization.
const TEXT_SIZE: u16 = 20;
const SMALL_TEXT_SIZE: u16 = 18;
const ELEMENT_SPACING: u16 = 10;
const TEXT_INPUT_PADDING: u16 = 4;
// ----------------------------------------------
const PATH_SECTION_LEFT_SIZE_PORTION: u16 = 3;
const PATH_SECTION_RIGHT_SIZE_PORTION: u16 = 7;
// ----------------------------------------------
const ELEMENT_NAME_SECTION_LEFT_SIZE_PORTION: u16 = 1;
const ELEMENT_NAME_SECTION_RIGHT_SIZE_PORTION: u16 = 4;
// ----------------------------------------------
const REMOVE_BUTTON_PORTION: u16 = 1;
const LIST_ITEM_PORTION: u16 = 5;

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

impl Default for VAlign {
    fn default() -> Self {
        Self::Center
    }
}

impl Default for HAlign {
    fn default() -> Self {
        Self::Center
    }
}

impl std::fmt::Display for VAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VAlign::Top => "top",
                VAlign::Center => "center",
                VAlign::Bottom => "bottom",
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
                HAlign::Left => "left",
                HAlign::Center => "center",
                HAlign::Right => "right",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterType {
    Any,
    Int,
    Bool,
    String,
    Float,
}

impl Default for ParameterType {
    fn default() -> Self {
        Self::Any
    }
}

impl std::fmt::Display for ParameterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParameterType::Any => "any",
                ParameterType::Int => "int",
                ParameterType::Bool => "bool",
                ParameterType::String => "string",
                ParameterType::Float => "float",
            }
        )
    }
}

impl ParameterType {
    const ALL: [ParameterType; 5] = [
        ParameterType::Any,
        ParameterType::Int,
        ParameterType::Bool,
        ParameterType::String,
        ParameterType::Float,
    ];
}

#[derive(Default, Clone)]
pub struct UiParameter {
    pub name: String,
    pub description: String,
    pub type_: Option<ParameterType>, // using underscore because `type` is a keyword
}

/// Function or Event
#[derive(Default, Clone)]
pub struct UiRunnable {
    pub name: String,
    pub parameters: Vec<UiParameter>, // array of pairs: name - description
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
    EntityListAddParameterClicked(usize), // item index
    GenerateClicked,
    AdditionalGfxExportArgsChanged(String),
    UiElementsTextChanged(String),
    UiElementTextChanged(String),
    GfxLayerTextChanged(String),
    EntityListItemChanged(usize, String), // item index, item name
    EntityListRemoveItem(usize),
    EntityListParameterNameChanged(usize, usize, String), // item index, param index, param name
    EntityListRemoveParameter(usize, usize),              // item index, param index
    EntityListParameterDescriptionChanged(usize, usize, String), // item index, param index, param desc
    EntityListParameterTypeChanged(usize, usize, ParameterType), // item index, param index, param type
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
    functions: Vec<UiRunnable>,
    events: Vec<UiRunnable>,
    halign: Option<HAlign>,
    valign: Option<VAlign>,
    gfx_layer: usize,
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
            halign: Some(HAlign::default()),
            valign: Some(VAlign::default()),
            gfx_layer: 0,
            fullscreen: false,
        }
    }

    pub fn view(&self) -> Element<MainLayoutMessage, Renderer<Theme>> {
        // Prepare buttons for categories.
        let mut functions_button = Button::new(Text::new("Functions").size(TEXT_SIZE))
            .on_press(MainLayoutMessage::ShowFunctions)
            .style(style::Button::Inactive)
            .width(Length::Fill);
        let mut events_button = Button::new(Text::new("Events").size(TEXT_SIZE))
            .on_press(MainLayoutMessage::ShowEvents)
            .style(style::Button::Inactive)
            .width(Length::Fill);

        // Highlight active.
        match self.current_list {
            EntityList::Functions => {
                functions_button = functions_button.style(style::Button::Default)
            }
            EntityList::Events => events_button = events_button.style(style::Button::Default),
        }

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
                    .push(
                        PickList::new(
                            &HAlign::ALL[..],
                            self.halign,
                            MainLayoutMessage::HorizontalAlignChanged,
                        )
                        .text_size(TEXT_SIZE),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Text::new("Vertical alignment:")
                            .size(TEXT_SIZE)
                            .vertical_alignment(Vertical::Center),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        PickList::new(
                            &VAlign::ALL[..],
                            self.valign,
                            MainLayoutMessage::VerticalAlignChanged,
                        )
                        .text_size(TEXT_SIZE),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Checkbox::new(
                            self.fullscreen,
                            "Fullscreen",
                            MainLayoutMessage::FullscreenChanged,
                        )
                        .text_size(TEXT_SIZE),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(Text::new("GFx layer").size(TEXT_SIZE))
                    .spacing(ELEMENT_SPACING)
                    .push(
                        TextInput::new(
                            "",
                            &self.gfx_layer.to_string(),
                            MainLayoutMessage::GfxLayerTextChanged,
                        )
                        .padding(TEXT_INPUT_PADDING)
                        .size(TEXT_SIZE),
                    ),
            )
            .spacing(ELEMENT_SPACING)
            .push(
                Row::new()
                    .push(functions_button)
                    .spacing(ELEMENT_SPACING)
                    .push(events_button),
            )
            .spacing(ELEMENT_SPACING)
            .push(self.get_entity_list())
            .spacing(ELEMENT_SPACING)
            .spacing(ELEMENT_SPACING)
            .push(
                Button::new(Text::new("Generate .gfx and .xml files").size(TEXT_SIZE))
                    .on_press(MainLayoutMessage::GenerateClicked)
                    .width(Length::Fill),
            )
            .push(
                Text::new(format!(
                    "v{}, by Alexander Tretyakov",
                    env!("CARGO_PKG_VERSION")
                ))
                .size(10),
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
            MainLayoutMessage::SelectPathToSwfFile => self.select_swf_file_path(app_config),
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
            MainLayoutMessage::EntityListRemoveParameter(item_index, param_index) => {
                self.remove_list_parameter(item_index, param_index)
            }
            MainLayoutMessage::EntityListParameterNameChanged(item_index, param_index, newname) => {
                self.update_list_parameter_name(item_index, param_index, newname)
            }
            MainLayoutMessage::EntityListParameterTypeChanged(item_index, param_index, type_) => {
                self.update_list_parameter_type(item_index, param_index, type_)
            }
            MainLayoutMessage::EntityListParameterDescriptionChanged(
                item_index,
                param_index,
                newname,
            ) => self.update_list_parameter_description(item_index, param_index, newname),
            MainLayoutMessage::EntityListAddParameterClicked(item_index) => {
                self.add_list_item_parameter(item_index)
            }
            MainLayoutMessage::GfxLayerTextChanged(gfx_layer) => self.update_gfx_layer(gfx_layer),
        }

        Command::none()
    }

    fn get_entity_list(&self) -> Element<MainLayoutMessage, Renderer<Theme>> {
        let mut list = Column::new();

        // Get reference to vector to use.
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
            // Collect parameters.
            let mut params = Column::new();
            for (param_index, parameter) in item.parameters.iter().enumerate() {
                params = params.push(
                            Row::new()
                                    .push(PickList::new(
                                &ParameterType::ALL[..],
                                parameter.type_,
                            move |type_: ParameterType| -> MainLayoutMessage {
                                    MainLayoutMessage::EntityListParameterTypeChanged(index, param_index, type_)
                                },
                                ))
                                .spacing(ELEMENT_SPACING)
                                .push(
                                    Column::new().push(
                                        TextInput::new(
                                            "",
                                            &parameter.name,
                                            move |name: String| -> MainLayoutMessage {
                                                MainLayoutMessage::EntityListParameterNameChanged(
                                                    index,
                                                    param_index,
                                                    name,
                                                )
                                            },
                                        )
                                        .size(TEXT_SIZE)
                                        .padding(TEXT_INPUT_PADDING),
                                    )
                                    .spacing(ELEMENT_SPACING)
                                    .push(
                                        TextInput::new(
                                            "",
                                            &parameter.description,
                                            move |name: String| -> MainLayoutMessage {
                                                MainLayoutMessage::EntityListParameterDescriptionChanged(
                                                    index,
                                                    param_index,
                                                    name,
                                                )
                                            },
                                        )
                                        .size(TEXT_SIZE)
                                        .padding(TEXT_INPUT_PADDING),
                                    )
                                    .width(Length::FillPortion(LIST_ITEM_PORTION)),
                                )
                                .spacing(ELEMENT_SPACING)
                                .push(
                                    Button::new(Text::new("Remove parameter").size(TEXT_SIZE))
                                        .on_press(MainLayoutMessage::EntityListRemoveParameter(index, param_index))
                                        .width(Length::FillPortion(REMOVE_BUTTON_PORTION)),
                                ),
                        );
            }

            // Add "Add parameter" button.
            params = params.spacing(ELEMENT_SPACING).push(
                Button::new(
                    Text::new("Add parameter")
                        .size(TEXT_SIZE)
                        .horizontal_alignment(Horizontal::Center),
                )
                .on_press(MainLayoutMessage::EntityListAddParameterClicked(index))
                .width(Length::Fill),
            );

            // Add item to list.
            list = list.push(
                Row::new()
                    .push(
                        Column::new()
                            .push(
                                TextInput::new(
                                    "",
                                    &item.name,
                                    move |name: String| -> MainLayoutMessage {
                                        MainLayoutMessage::EntityListItemChanged(index, name)
                                    },
                                )
                                .size(TEXT_SIZE)
                                .padding(TEXT_INPUT_PADDING),
                            )
                            .spacing(ELEMENT_SPACING)
                            .push(params)
                            .width(Length::FillPortion(LIST_ITEM_PORTION)),
                    )
                    .spacing(ELEMENT_SPACING)
                    .push(
                        Button::new(Text::new("Remove item").size(TEXT_SIZE))
                            .on_press(MainLayoutMessage::EntityListRemoveItem(index))
                            .width(Length::FillPortion(REMOVE_BUTTON_PORTION)),
                    ),
            );
        }

        // Add "Add" button to list.
        list = list.spacing(ELEMENT_SPACING).push(
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

    /// Check whether the specified directory contains an XML file with
    /// the specified name.
    ///
    /// ## Arguments
    /// * `path_to_dir`: path to the directory to check for an XML file.
    /// * `name`: name of the file (without extension) to check.
    ///
    /// ## Return
    /// `Err()` if not found, otherwise `Ok(String)` with a full path to the found file.
    fn check_if_xml_exists(&self, path_to_dir: &str, name: &str) -> Result<String, ()> {
        let mut path = Path::new(path_to_dir).to_path_buf();

        if !path.exists() || !path.is_dir() {
            return Err(());
        }

        path.push(&format!("{}.xml", name));

        if !path.exists() {
            return Err(());
        }

        Ok(path.to_string_lossy().to_string())
    }

    fn are_all_required_fields_filled(&self) -> bool {
        if self.path_to_gfxexport_bin.is_empty() {
            Self::show_message_about_empty_field("Path to GFxExport");
            return false;
        }

        if self.path_to_swf_file.is_empty() {
            Self::show_message_about_empty_field("Path to .swf file");
            return false;
        }

        if self.path_to_gfx_dir.is_empty() {
            Self::show_message_about_empty_field("Output directory for .gfx files");
            return false;
        }

        if self.path_to_xml_dir.is_empty() {
            Self::show_message_about_empty_field("Output directory for .xml files");
            return false;
        }

        if self.ui_elements_name.is_empty() {
            Self::show_message_about_empty_field("Elements name");
            return false;
        }

        if self.ui_element_name.is_empty() {
            Self::show_message_about_empty_field("Element name");
            return false;
        }

        return true;
    }

    fn show_message_about_empty_field(field_name: &str) {
        MessageDialog::new()
            .set_type(MessageType::Error)
            .set_title("Error")
            .set_text(&format!("Field \"{}\" must be filled.", field_name))
            .show_alert()
            .unwrap();
    }

    fn get_data_from_existing_xml(&mut self, path_to_xml_file: &str) {
        if !Path::new(path_to_xml_file).exists() {
            return;
        }

        // Ask if the user wants to read this file.
        let yes = MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Info")
            .set_text(&format!(
                "The XML file \"{}\" already exists, do you want \
                        to get data from this file here?",
                &path_to_xml_file
            ))
            .show_confirm()
            .unwrap();
        if !yes {
            return;
        }

        // Parse XML file.
        let result = XmlManager::read_config(path_to_xml_file);
        if let Err(app_error) = result {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(&format!(
                    "Failed to parse XML file at \"{}\". Error: {}",
                    path_to_xml_file, app_error
                ))
                .show_alert()
                .unwrap();
            return;
        }
        let config = result.unwrap();

        // Update values from config.
        self.ui_elements_name = config.ui_elements_name;
        self.ui_element_name = config.ui_element_name;
        self.fullscreen = config.fullscreen;
        self.gfx_layer = config.gfx_layer;
        self.halign = Some(config.halign);
        self.valign = Some(config.valign);
        self.functions = config.functions;
        self.events = config.events;
    }

    fn generate(&mut self, app_config: &mut ApplicationConfig) {
        if !self.are_all_required_fields_filled() {
            return;
        }

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

        // Check .swf file name.
        if Path::new(&self.path_to_swf_file).file_stem().is_none() {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text("*.swf file should have a file name.")
                .show_alert()
                .unwrap();
            return;
        }

        let file_name = Path::new(&self.path_to_swf_file)
            .file_stem()
            .unwrap()
            .to_string_lossy();

        // Construct path to output .xml file.
        let mut path_to_xml_file = Path::new(&self.path_to_xml_dir).to_path_buf();
        path_to_xml_file.push(format!("{}.xml", file_name.to_string()));

        // Construct path to output .gfx file.
        let mut path_to_gfx_file = Path::new(&self.path_to_gfx_dir).to_path_buf();
        path_to_gfx_file.push(format!("{}.gfx", file_name.to_string()));

        // Check if .xml file already exists.
        if path_to_xml_file.exists() {
            let yes = MessageDialog::new()
                .set_type(MessageType::Warning)
                .set_title("Warning")
                .set_text(&format!(
                    "Output .xml file \"{}\" already exists, do you want to overwrite it?",
                    path_to_xml_file.to_string_lossy().to_string()
                ))
                .show_confirm()
                .unwrap();
            if !yes {
                return;
            }
        }

        // Make sure output directories exist.
        if !Path::new(&self.path_to_xml_dir).exists() {
            if let Err(e) = fs::create_dir_all(&self.path_to_xml_dir) {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title("Error")
                    .set_text(&format!(
                        "Failed to create output directory for .xml files, error: {}",
                        e
                    ))
                    .show_alert()
                    .unwrap();
                return;
            }
        }
        if !Path::new(&self.path_to_gfx_dir).exists() {
            if let Err(e) = fs::create_dir_all(&self.path_to_gfx_dir) {
                MessageDialog::new()
                    .set_type(MessageType::Error)
                    .set_title("Error")
                    .set_text(&format!(
                        "Failed to create output directory for .gfx files, error: {}",
                        e
                    ))
                    .show_alert()
                    .unwrap();
                return;
            }
        }

        // Construct config.
        let config = XmlConfig {
            ui_elements_name: self.ui_elements_name.clone(),
            ui_element_name: self.ui_element_name.clone(),
            gfx_file_name: format!("{}.gfx", file_name),
            gfx_layer: self.gfx_layer,
            fullscreen: self.fullscreen,
            halign: self.halign.unwrap_or(HAlign::Center),
            valign: self.valign.unwrap_or(VAlign::Center),
            functions: self.functions.clone(),
            events: self.events.clone(),
        };

        // Write to file.
        if let Err(app_error) = XmlManager::write_config(
            config,
            path_to_xml_file.to_string_lossy().to_string().as_str(),
        ) {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(&format!("Failed to write .xml file, error: {}", app_error))
                .show_alert()
                .unwrap();
            return;
        }

        // Split GFxExport additional arguments.
        let mut args = self
            .additional_gfxexport_args
            .split_ascii_whitespace()
            .collect::<Vec<&str>>();
        args.push("-d"); // specify output directory
        args.push(&self.path_to_gfx_dir);

        // Merge arguments into one string to show to user.
        let mut args_to_show = String::new();
        for arg in args.iter() {
            args_to_show += &format!("\"{}\" ", arg);
        }

        // Run GFxExport.
        if let Err(e) = process::Command::new(&self.path_to_gfxexport_bin)
            .arg(&self.path_to_swf_file)
            .args(args)
            .output()
        {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(&format!("GFxExport failed, error: {}", e))
                .show_alert()
                .unwrap();
            return;
        }

        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Info")
            .set_text(&format!(
                "Successfully generated .gfx and .xml files.\n\n\
                Output .xml file: {}\n\n\
                Output .gfx file: {}\n\n\
                Used GFxExport arguments: \"{}\" {}",
                path_to_xml_file.to_string_lossy(),
                path_to_gfx_file.to_string_lossy(),
                &self.path_to_swf_file,
                &args_to_show
            ))
            .show_alert()
            .unwrap();
    }

    fn update_additional_gfxexport_args(&mut self, args: String) {
        self.additional_gfxexport_args = args;
    }

    fn update_list_item(&mut self, index: usize, newname: String) {
        match self.current_list {
            EntityList::Functions => {
                self.functions[index].name = newname;
            }
            EntityList::Events => {
                self.events[index].name = newname;
            }
        }
    }

    fn update_list_parameter_type(
        &mut self,
        item_index: usize,
        param_index: usize,
        type_: ParameterType,
    ) {
        match self.current_list {
            EntityList::Functions => {
                self.functions[item_index].parameters[param_index].type_ = Some(type_);
            }
            EntityList::Events => {
                self.events[item_index].parameters[param_index].type_ = Some(type_);
            }
        }
    }

    fn update_list_parameter_name(
        &mut self,
        item_index: usize,
        param_index: usize,
        newname: String,
    ) {
        match self.current_list {
            EntityList::Functions => {
                self.functions[item_index].parameters[param_index].name = newname;
            }
            EntityList::Events => {
                self.events[item_index].parameters[param_index].name = newname;
            }
        }
    }

    fn update_list_parameter_description(
        &mut self,
        item_index: usize,
        param_index: usize,
        newname: String,
    ) {
        match self.current_list {
            EntityList::Functions => {
                self.functions[item_index].parameters[param_index].description = newname;
            }
            EntityList::Events => {
                self.events[item_index].parameters[param_index].description = newname;
            }
        }
    }

    fn remove_list_parameter(&mut self, item_index: usize, param_index: usize) {
        match self.current_list {
            EntityList::Functions => {}
            EntityList::Events => {
                self.events[item_index].parameters.remove(param_index);
            }
        }
    }

    fn add_list_item_parameter(&mut self, item_index: usize) {
        let parameter = UiParameter {
            name: String::from("Parameter name"),
            description: String::from("Parameter description"),
            type_: Some(ParameterType::Any),
        };

        match self.current_list {
            EntityList::Functions => {
                self.functions[item_index].parameters.push(parameter);
            }
            EntityList::Events => {
                self.events[item_index].parameters.push(parameter);
            }
        }
    }

    fn add_list_item(&mut self) {
        match self.current_list {
            EntityList::Functions => self.functions.push(UiRunnable {
                name: String::from("Function name"),
                parameters: Vec::new(),
            }),
            EntityList::Events => self.events.push(UiRunnable {
                name: String::from("Event name"),
                parameters: Vec::new(),
            }),
        }
    }

    fn remove_list_item(&mut self, index: usize) {
        match self.current_list {
            EntityList::Functions => {
                self.functions.remove(index);
            }
            EntityList::Events => {
                self.events.remove(index);
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

    fn update_gfx_layer(&mut self, gfx_layer: String) {
        let result = gfx_layer.parse::<usize>();
        if let Err(e) = result {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(&format!(
                    "Failed to convert GFx layer to unsigned integer. Error: {}",
                    e,
                ))
                .show_alert()
                .unwrap();
            return;
        }
        self.gfx_layer = result.unwrap();
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

        // See if an XML file exists.
        if Path::new(&self.path_to_swf_file).exists() {
            let swf_file_name = Path::new(&self.path_to_swf_file).file_stem();
            if swf_file_name.is_none() {
                return;
            }
            let swf_file_name = swf_file_name.unwrap().to_string_lossy().to_string();

            let result = self.check_if_xml_exists(&self.path_to_xml_dir, &swf_file_name);
            if result.is_err() {
                return;
            }
            let xml_path = result.unwrap();

            self.get_data_from_existing_xml(&xml_path);
        }
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

    fn select_swf_file_path(&mut self, app_config: &mut ApplicationConfig) {
        // Get path to .swf file.
        let path = FileDialog::new()
            .set_location(&app_config.last_used_swf_dir)
            .add_filter("SWF Movie", &["swf"])
            .show_open_single_file()
            .unwrap();
        if path.is_none() {
            return;
        }
        let path = path.unwrap();

        // Save.
        self.path_to_swf_file = path.to_string_lossy().to_string();

        // Set UI elemnt names.
        self.ui_elements_name = path.file_stem().unwrap().to_string_lossy().to_string();
        self.ui_element_name = self.ui_elements_name.clone();

        // Save paths to output directies.
        if path.parent().is_some() && path.parent().unwrap().parent().is_some() {
            // Set path to .gfx and .xml files.
            let path_to_gfx = path.parent().unwrap().parent().unwrap();
            let mut path_to_xml = path_to_gfx.to_path_buf();
            path_to_xml.push("UIElements");

            self.path_to_gfx_dir = path_to_gfx.to_string_lossy().to_string();
            self.path_to_xml_dir = path_to_xml.to_string_lossy().to_string();

            // Save directory to config.
            app_config.last_used_swf_dir = path.parent().unwrap().to_string_lossy().to_string();
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

        // See if an XML file exists.
        if Path::new(&self.path_to_swf_file).exists() {
            let swf_file_name = Path::new(&self.path_to_swf_file).file_stem();
            if swf_file_name.is_none() {
                return;
            }
            let swf_file_name = swf_file_name.unwrap().to_string_lossy().to_string();

            let result = self.check_if_xml_exists(&self.path_to_xml_dir, &swf_file_name);
            if result.is_err() {
                return;
            }
            let xml_path = result.unwrap();

            self.get_data_from_existing_xml(&xml_path);
        }
    }

    fn select_gfx_bin_path(&mut self, app_config: &mut ApplicationConfig) {
        #[cfg(windows)]
        {
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
        }

        #[cfg(not(windows))]
        {
            // Get path to GFxExport1 file.
            let path = FileDialog::new().show_open_single_file().unwrap();
            if path.is_none() {
                return;
            }
            let path = path.unwrap();

            // Save to UI.
            self.path_to_gfxexport_bin = path.to_string_lossy().to_string();
        }

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
