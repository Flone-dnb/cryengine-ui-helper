#![deny(warnings)]
// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use iced::window::Position;
use iced::{application, executor, window, Application, Color, Command, Element, Settings, Size};
use iced::{theme, Theme};

// Custom.
use layouts::main_layout::*;
use misc::config::ApplicationConfig;

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
        Theme::Dark
    }

    fn style(&self) -> theme::Application {
        theme::Application::Custom(|_theme| application::Appearance {
            background_color: Color::BLACK,
            text_color: Color::WHITE,
        })
    }

    fn title(&self) -> String {
        String::from("CRYENGINE UI Helper")
    }

    fn view(&self) -> Element<ApplicationMessage> {
        match self.current_layout {
            Layout::Main => self
                .main_layout
                .view()
                .map(move |message| ApplicationMessage::MainLayoutMessage(message)),
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
        height: 500,
    };

    // Prepare window settings.
    let mut window_settings = window::Settings::default();
    window_settings.size = (window_size.width, window_size.height);
    window_settings.position = Position::Centered;

    ApplicationState::run(Settings {
        antialiasing: true,
        window: window_settings,
        ..Settings::default()
    })
}
