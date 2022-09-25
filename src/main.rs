#![deny(warnings)]
// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use iced::window::Position;
use iced::{executor, window, Application, Command, Element, Renderer, Settings, Size};

// Custom.
use layouts::main_layout::*;
use misc::config::ApplicationConfig;
use misc::theme::Theme;

mod layouts;
mod managers;
mod misc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    Main,
}

#[derive(Debug, Clone)]
pub enum ApplicationMessage {
    MainLayoutMessage(MainLayoutMessage),
}

pub struct ApplicationState {
    current_layout: Layout,

    main_layout: MainLayout,

    app_config: ApplicationConfig,
}

impl Application for ApplicationState {
    type Message = ApplicationMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<ApplicationMessage>) {
        let config = ApplicationConfig::new();
        (
            Self {
                current_layout: Layout::Main,
                main_layout: MainLayout::new(&config),
                app_config: config,
            },
            Command::none(),
        )
    }

    fn theme(&self) -> Theme {
        Theme::DarkOrange
    }

    fn title(&self) -> String {
        String::from("CRYENGINE UI Helper")
    }

    fn view(&self) -> Element<ApplicationMessage, Renderer<Theme>> {
        match self.current_layout {
            Layout::Main => self
                .main_layout
                .view()
                .map(ApplicationMessage::MainLayoutMessage),
        }
    }

    fn update(&mut self, message: ApplicationMessage) -> Command<ApplicationMessage> {
        match message {
            ApplicationMessage::MainLayoutMessage(message) => {
                self.main_layout.update(message, &mut self.app_config)
            }
        }
    }
}

fn main() -> iced::Result {
    let window_size = Size {
        width: 900,
        height: 700,
    };

    // Prepare window settings.
    let window_settings = window::Settings {
        size: (window_size.width, window_size.height),
        position: Position::Centered,
        ..window::Settings::default()
    };

    ApplicationState::run(Settings {
        antialiasing: true,
        window: window_settings,
        ..Settings::default()
    })
}
