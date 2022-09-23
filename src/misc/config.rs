// Std.
use std::fs::create_dir_all;
use std::path::PathBuf;

// External.
use configparser::ini::Ini;
use platform_dirs::AppDirs;

// Custom.
use super::error::*;

const CONFIG_DIR_NAME: &str = "CRYENGINE UI Helper";
const CONFIG_FILE_NAME: &str = "config.ini";
const CONFIG_PATHS_SECTION_NAME: &str = "paths";
const CONFIG_PATH_TO_GFXEXPORT_BIN: &str = "path_to_gfxexport_bin";

#[derive(Debug)]
pub struct ApplicationConfig {
    pub path_to_gfxexport_bin: String,
}

impl ApplicationConfig {
    pub fn new() -> Self {
        let mut app_config = ApplicationConfig::default();

        // Try reading config from .ini file.
        let mut config = Ini::new();
        let config_path = Self::get_config_file_path();
        let map = config.load(&config_path);
        if map.is_err() {
            // No file found, create a new file.
            if let Err(e) = app_config.save() {
                // Non-critical error.
                print!("WARNING: {}", AppError::new(&e.to_string()));
            }
            return app_config;
        }

        let mut some_values_were_empty = false;

        // Read config.
        let path_to_gfxexport_bin =
            config.get(CONFIG_PATHS_SECTION_NAME, CONFIG_PATH_TO_GFXEXPORT_BIN);
        if path_to_gfxexport_bin.is_none() {
            some_values_were_empty = true;
        } else {
            app_config.path_to_gfxexport_bin = path_to_gfxexport_bin.unwrap();
        }

        // Resave if needed.
        if some_values_were_empty {
            // Create a new file with all values filled.
            if let Err(e) = app_config.save() {
                // Non-critical error.
                print!("WARNING: {}", AppError::new(&e.to_string()));
            }
        }

        app_config
    }

    pub fn save(&self) -> Result<(), AppError> {
        let mut config = Ini::new();

        config.setstr(
            CONFIG_PATHS_SECTION_NAME,
            CONFIG_PATH_TO_GFXEXPORT_BIN,
            Some(&self.path_to_gfxexport_bin),
        );

        if let Err(e) = config.write(Self::get_config_file_path()) {
            return Err(AppError::new(&e.to_string()));
        }

        Ok(())
    }

    pub fn get_config_file_path() -> PathBuf {
        #[cfg(any(windows, unix))]
        {
            let app_dirs = AppDirs::new(Some(CONFIG_DIR_NAME), true).unwrap_or_else(|| {
                panic!(
                    "An error occurred at [{}, {}]: can't read user dirs.",
                    file!(),
                    line!(),
                )
            });

            let mut config_path = app_dirs.config_dir;

            // Create directory if not exists.
            if !config_path.exists() {
                if let Err(e) = create_dir_all(&config_path) {
                    panic!("An error occurred at [{}, {}]: {:?}", file!(), line!(), e);
                }
            }

            config_path.push(CONFIG_FILE_NAME);
            config_path
        }
        #[cfg(not(any(windows, unix)))]
        {
            compile_error!("Client is not implemented for this OS.");
        }
    }
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        ApplicationConfig {
            path_to_gfxexport_bin: String::new(),
        }
    }
}