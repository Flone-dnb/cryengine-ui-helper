use std::fs::OpenOptions;
use std::io::{Cursor, Write};

use quick_xml::Writer;
// External.
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::reader::Reader;

// Custom.
use crate::layouts::main_layout::{HAlign, ParameterType, UiParameter, UiRunnable, VAlign};
use crate::misc::error::AppError;

#[derive(Default)]
pub struct XmlConfig {
    pub ui_elements_name: String,
    pub ui_element_name: String,
    pub gfx_file_name: String,
    pub gfx_layer: usize,
    pub fullscreen: bool,
    pub halign: HAlign,
    pub valign: VAlign,
    pub functions: Vec<UiRunnable>,
    pub events: Vec<UiRunnable>,
}

pub struct XmlManager;

impl XmlManager {
    pub fn write_config(config: XmlConfig, path_to_config: &str) -> Result<(), AppError> {
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 4);

        // Write <UIElements> tag.
        let mut element = BytesStart::new("UIElements");
        element.push_attribute(("name", config.ui_elements_name.as_str()));
        if let Err(e) = writer.write_event(Event::Start(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        // Write <UIElement> tag.
        let mut element = BytesStart::new("UIElement");
        element.push_attribute(("name", config.ui_element_name.as_str()));
        if let Err(e) = writer.write_event(Event::Start(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        // Write <GFx> tag.
        let mut element = BytesStart::new("GFx");
        element.push_attribute(("file", config.gfx_file_name.as_str()));
        element.push_attribute(("layer", config.gfx_layer.to_string().as_str()));
        if let Err(e) = writer.write_event(Event::Start(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        // Write <Constraints> tag.
        let element = BytesStart::new("Constraints");
        if let Err(e) = writer.write_event(Event::Start(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        // Write <Align> tag.
        let mut element = BytesStart::new("Align");
        if config.fullscreen {
            element.push_attribute(("mode", "fullscreen"));
            element.push_attribute(("scale", "1"));
            element.push_attribute(("maximize", "1"));
        } else {
            element.push_attribute(("mode", "dynamic"));
            element.push_attribute(("valign", config.valign.to_string().as_str()));
            element.push_attribute(("halign", config.halign.to_string().as_str()));
        }
        if let Err(e) = writer.write_event(Event::Empty(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        // Write </Constraints> tag.
        let element = BytesEnd::new("Constraints");
        if let Err(e) = writer.write_event(Event::End(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        // Write </GFx> tag.
        let element = BytesEnd::new("GFx");
        if let Err(e) = writer.write_event(Event::End(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        if !config.functions.is_empty() {
            // Write <functions> tag.
            let element = BytesStart::new("functions");
            if let Err(e) = writer.write_event(Event::Start(element)) {
                return Err(AppError::new(&e.to_string()));
            }

            for function in config.functions.iter() {
                if function.parameters.is_empty() {
                    // Write <function> tag.
                    let mut element = BytesStart::new("function");
                    element.push_attribute(("name", function.name.as_str()));
                    element.push_attribute(("funcname", function.name.as_str()));
                    if let Err(e) = writer.write_event(Event::Empty(element)) {
                        return Err(AppError::new(&e.to_string()));
                    }

                    continue;
                }

                // Write <function> tag.
                let mut element = BytesStart::new("function");
                element.push_attribute(("name", function.name.as_str()));
                element.push_attribute(("funcname", function.name.as_str()));
                if let Err(e) = writer.write_event(Event::Start(element)) {
                    return Err(AppError::new(&e.to_string()));
                }

                for parameter in function.parameters.iter() {
                    // Write <param> tag.
                    let mut element = BytesStart::new("param");
                    element.push_attribute(("name", parameter.name.as_str()));
                    if !parameter.description.is_empty() {
                        element.push_attribute(("desc", parameter.description.as_str()));
                    }
                    if parameter.type_.is_some() && parameter.type_ != Some(ParameterType::Any) {
                        element.push_attribute((
                            "type",
                            parameter.type_.unwrap().to_string().as_str(),
                        ));
                    }

                    if let Err(e) = writer.write_event(Event::Empty(element)) {
                        return Err(AppError::new(&e.to_string()));
                    }
                }

                // Write </function> tag.
                let element = BytesEnd::new("function");
                if let Err(e) = writer.write_event(Event::End(element)) {
                    return Err(AppError::new(&e.to_string()));
                }
            }

            // Write </functions> tag.
            let element = BytesEnd::new("functions");
            if let Err(e) = writer.write_event(Event::End(element)) {
                return Err(AppError::new(&e.to_string()));
            }
        }

        if !config.events.is_empty() {
            // Write <events> tag.
            let element = BytesStart::new("events");
            if let Err(e) = writer.write_event(Event::Start(element)) {
                return Err(AppError::new(&e.to_string()));
            }

            for event in config.events.iter() {
                if event.parameters.is_empty() {
                    // Write <event> tag.
                    let mut element = BytesStart::new("event");
                    element.push_attribute(("name", event.name.as_str()));
                    element.push_attribute(("fscommand", event.name.as_str()));
                    if let Err(e) = writer.write_event(Event::Empty(element)) {
                        return Err(AppError::new(&e.to_string()));
                    }

                    continue;
                }

                // Write <event> tag.
                let mut element = BytesStart::new("event");
                element.push_attribute(("name", event.name.as_str()));
                element.push_attribute(("fscommand", event.name.as_str()));
                if let Err(e) = writer.write_event(Event::Start(element)) {
                    return Err(AppError::new(&e.to_string()));
                }

                for parameter in event.parameters.iter() {
                    // Write <param> tag.
                    let mut element = BytesStart::new("param");
                    element.push_attribute(("name", parameter.name.as_str()));
                    if !parameter.description.is_empty() {
                        element.push_attribute(("desc", parameter.description.as_str()));
                    }
                    if parameter.type_.is_some() && parameter.type_ != Some(ParameterType::Any) {
                        element.push_attribute((
                            "type",
                            parameter.type_.unwrap().to_string().as_str(),
                        ));
                    }

                    if let Err(e) = writer.write_event(Event::Empty(element)) {
                        return Err(AppError::new(&e.to_string()));
                    }
                }

                // Write </event> tag.
                let element = BytesEnd::new("event");
                if let Err(e) = writer.write_event(Event::End(element)) {
                    return Err(AppError::new(&e.to_string()));
                }
            }

            // Write </events> tag.
            let element = BytesEnd::new("events");
            if let Err(e) = writer.write_event(Event::End(element)) {
                return Err(AppError::new(&e.to_string()));
            }
        }

        // Write </UIElement> tag.
        let element = BytesEnd::new("UIElement");
        if let Err(e) = writer.write_event(Event::End(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        // Write </UIElements> tag.
        let element = BytesEnd::new("UIElements");
        if let Err(e) = writer.write_event(Event::End(element)) {
            return Err(AppError::new(&e.to_string()));
        }

        let result = writer.into_inner().into_inner();

        // Write result to file.
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(path_to_config);
        if let Err(e) = file {
            return Err(AppError::new(&e.to_string()));
        }
        let mut file = file.unwrap();

        if let Err(e) = file.write_all(&result) {
            return Err(AppError::new(&e.to_string()));
        }

        Ok(())
    }
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
