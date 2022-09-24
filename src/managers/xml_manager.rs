// External.
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

// Custom.
use crate::layouts::main_layout::{HAlign, ParameterType, UiParameter, UiRunnable, VAlign};
use crate::misc::error::AppError;

#[derive(Default)]
pub struct XmlConfig {
    pub ui_elements_name: String,
    pub ui_element_name: String,
    pub gfx_layer: usize,
    pub fullscreen: bool,
    pub halign: HAlign,
    pub valign: VAlign,
    pub functions: Vec<UiRunnable>,
    pub events: Vec<UiRunnable>,
}

pub struct XmlManager;

impl XmlManager {
    pub fn read_config(path_to_config: &str) -> Result<XmlConfig, AppError> {
        let reader = Reader::from_file(path_to_config);
        if let Err(e) = reader {
            return Err(AppError::new(&e.to_string()));
        }
        let mut reader = reader.unwrap();
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut config = XmlConfig::default();
        let mut current_item_name = String::new();
        let mut is_in_function_event = false;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => {
                    return Err(AppError::new(&e.to_string()));
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(event)) | Ok(Event::Empty(event)) => match event.name().as_ref() {
                    b"UIElements" => {
                        config.ui_elements_name = Self::get_attribute_value(&event, "name")?;
                    }
                    b"UIElement" => {
                        config.ui_element_name = Self::get_attribute_value(&event, "name")?;
                    }
                    b"GFx" => {
                        let layer = Self::get_attribute_value(&event, "layer")?;
                        let layer = layer.parse::<usize>();
                        if let Err(e) = layer {
                            return Err(AppError::new(&e.to_string()));
                        }
                        config.gfx_layer = layer.unwrap();
                    }
                    b"Align" => {
                        // Get mode.
                        let mode = Self::get_attribute_value(&event, "mode")?;
                        if mode == "fullscreen" {
                            config.fullscreen = true;
                        } else {
                            config.fullscreen = false;

                            // Get valign.
                            let valign = Self::get_attribute_value(&event, "valign")?;
                            let valign = valign.to_lowercase();
                            match valign.as_str() {
                                "top" => {
                                    config.valign = VAlign::Top;
                                }
                                "center" => {
                                    config.valign = VAlign::Center;
                                }
                                "bottom" => {
                                    config.valign = VAlign::Bottom;
                                }
                                _ => {}
                            }

                            // Get halign.
                            let halign = Self::get_attribute_value(&event, "halign")?;
                            let halign = halign.to_lowercase();
                            match halign.as_str() {
                                "left" => {
                                    config.halign = HAlign::Left;
                                }
                                "center" => {
                                    config.halign = HAlign::Center;
                                }
                                "right" => {
                                    config.halign = HAlign::Right;
                                }
                                _ => {}
                            }
                        }
                    }
                    b"function" => {
                        is_in_function_event = true;
                        current_item_name = Self::get_attribute_value(&event, "name")?;
                        config.functions.push(UiRunnable {
                            name: current_item_name.clone(),
                            parameters: Vec::new(),
                        });
                    }
                    b"event" => {
                        is_in_function_event = false;
                        current_item_name = Self::get_attribute_value(&event, "name")?;
                        config.events.push(UiRunnable {
                            name: current_item_name.clone(),
                            parameters: Vec::new(),
                        });
                    }
                    b"param" => {
                        let name = Self::get_attribute_value(&event, "name")?;
                        let desc = Self::get_attribute_value(&event, "desc")?;
                        let type_ = Self::get_attribute_value(&event, "type");

                        let mut _vec_to_use = &mut config.events;
                        if is_in_function_event {
                            _vec_to_use = &mut config.functions;
                        }

                        for item in _vec_to_use.iter_mut() {
                            if &item.name == &current_item_name {
                                let mut parameter_type = ParameterType::Any;
                                if type_.is_ok() {
                                    // optional parameter
                                    match type_.unwrap().to_lowercase().as_str() {
                                        "int" => parameter_type = ParameterType::Int,
                                        "bool" => parameter_type = ParameterType::Bool,
                                        "string" => parameter_type = ParameterType::String,
                                        "float" => parameter_type = ParameterType::Float,
                                        _ => {}
                                    }
                                }

                                let parameter = UiParameter {
                                    name,
                                    description: desc,
                                    type_: Some(parameter_type),
                                };
                                item.parameters.push(parameter);

                                break;
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }

            buf.clear();
        }

        Ok(config)
    }

    fn get_attribute_value(event: &BytesStart, attribute_name: &str) -> Result<String, AppError> {
        // Get attribute data.
        let result = event.try_get_attribute(attribute_name);
        if let Err(e) = result {
            return Err(AppError::new(&e.to_string()));
        }
        let result = result.unwrap();
        if result.is_none() {
            return Err(AppError::new(&format!(
                "\"{}\" attribute not found",
                attribute_name,
            )));
        }

        // Get attribute value.
        let result = result.unwrap().unescape_value();
        if let Err(e) = result {
            return Err(AppError::new(&e.to_string()));
        }

        Ok(result.unwrap().to_string())
    }
}
