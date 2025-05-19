use std::fmt::{self, Display};

use iced::widget::{button, column, container, row, Button, Space};
use iced::{window, Alignment, Element, Length, Renderer, Settings, Task, Theme};
use iced_widget::graphics::{self, compositor};
use iced_winit::Program;

mod attack;
mod character;
mod level;
mod npc;
mod utils;

const DATA_PATH: &str = "../data/";

fn main() {
    let app = App {
        state: EditorState::NotSelected,
    };
    let window_settings = Some(window::Settings::default());

    iced_winit::program::run::<App, <Renderer as compositor::Default>::Compositor>(
        Settings::default().into(),
        graphics::Settings::default(),
        window_settings,
        app,
    )
    .expect("Should run the app");
}

#[derive(Debug, Clone)]
enum Message {
    SelectKind(EditorKind),
    Attack(attack::Message),
    Npc(npc::Message),
    Level(level::Message),
    Character(character::Message),
}

#[derive(Debug, Clone, PartialEq)]
enum EditorKind {
    Attack,
    Npc,
    Level,
    Character,
}

impl EditorKind {
    fn into_message(self) -> Message {
        Message::SelectKind(self)
    }
    fn label(&self) -> &'static str {
        use EditorKind::*;
        match self {
            Attack => "Attack",
            Npc => "Npc",
            Level => "Level",
            Character => "Character",
        }
    }
    fn make_button(self, selected: &Option<EditorKind>) -> Button<'static, Message> {
        let style = if let Some(selected) = selected {
            if selected == &self {
                button::primary
            } else {
                button::secondary
            }
        } else {
            button::secondary
        };
        button(self.label())
            .on_press(self.into_message())
            .style(style)
    }
}

impl Display for EditorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

enum EditorState {
    NotSelected,
    Attack(Box<attack::Page>),
    Npc(Box<npc::Page>),
    Level(Box<level::Page>),
    Character(Box<character::Page>),
}

struct App {
    state: EditorState,
}

impl Program for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Renderer = Renderer;
    type Flags = Self;

    fn new(app: Self) -> (Self, Task<Message>) {
        (app, Task::none())
    }
    fn title(&self, _window: window::Id) -> String {
        "Editor".into()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectKind(kind) => match kind {
                EditorKind::Attack => self.state = attack::load_state(),
                EditorKind::Npc => self.state = npc::load_state(),
                EditorKind::Level => self.state = level::load_state(),
                EditorKind::Character => self.state = character::load_state(),
            },
            Message::Attack(message) => {
                if let EditorState::Attack(attack) = &mut self.state {
                    attack.update(message);
                }
            }
            Message::Npc(message) => {
                if let EditorState::Npc(page) = &mut self.state {
                    page.update(message);
                }
            }
            Message::Level(message) => {
                if let EditorState::Level(page) = &mut self.state {
                    page.update(message);
                }
            }
            Message::Character(message) => {
                if let EditorState::Character(page) = &mut self.state {
                    page.update(message);
                }
            }
        }
        Task::none()
    }
    fn view(&self, _window: window::Id) -> Element<Message> {
        let selected_kind = self.selected_kind();
        let editor_kind_picker = row![
            EditorKind::Attack.make_button(&selected_kind),
            EditorKind::Npc.make_button(&selected_kind),
            EditorKind::Level.make_button(&selected_kind),
            EditorKind::Character.make_button(&selected_kind),
        ]
        .align_y(Alignment::Center)
        .spacing(10);
        let mut contents = column![].align_x(Alignment::Center).spacing(10);

        match &self.state {
            EditorState::NotSelected => (),
            EditorState::Attack(page) => {
                let element = page.view().map(Message::Attack);
                contents = contents.push(element);
            }
            EditorState::Npc(page) => {
                let element = page.view().map(Message::Npc);
                contents = contents.push(element);
            }
            EditorState::Level(page) => {
                let element = page.view().map(Message::Level);
                contents = contents.push(element);
            }
            EditorState::Character(page) => {
                let element = page.view().map(Message::Character);
                contents = contents.push(element);
            }
        }

        column![
            editor_kind_picker,
            Space::with_height(20),
            container(contents)
                .center_x(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .align_x(Alignment::Center)
        .padding(20)
        .spacing(10)
        .into()
    }
    fn theme(&self, _window: window::Id) -> Theme {
        Theme::TokyoNight
    }
}

impl App {
    fn selected_kind(&self) -> Option<EditorKind> {
        use EditorKind::*;
        match self.state {
            EditorState::NotSelected => return None,
            EditorState::Attack(_) => Attack,
            EditorState::Npc(_) => Npc,
            EditorState::Level(_) => Level,
            EditorState::Character(_) => Character,
        }
        .into()
    }
}
