use std::fmt::{self, Display};

use iced::widget::{column, container, pick_list, row, text};
use iced::{window, Alignment, Element, Length, Renderer, Settings, Task, Theme};
use iced_widget::graphics::{self, compositor};
use iced_winit::Program;

use shared::npc::NpcConstructor;

mod attack;
mod level;
mod npc;

use level::Level;

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
    EditAttack(u32),
    Attack(attack::item::Message),
    AttackList(attack::list::Message),
    Npc(npc::Message),
    Level(level::Message),
}

#[derive(Debug, Clone, PartialEq)]
enum EditorKind {
    Attack,
    AttackList,
    Npc,
    Level,
}

impl EditorKind {
    const fn items() -> [EditorKind; 4] {
        use EditorKind::*;
        [Attack, AttackList, Npc, Level]
    }
}

impl Display for EditorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

enum EditorState {
    NotSelected,
    Attack(Box<attack::item::Page>),
    AttackList(Box<attack::list::Page>),
    Npc(Box<Option<NpcConstructor>>),
    Level(Box<Option<Level>>),
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
                EditorKind::Attack => (), // do nothing here
                EditorKind::AttackList => {
                    self.state = EditorState::AttackList(Box::new(attack::list::Page::load()))
                }
                EditorKind::Npc => self.state = EditorState::Npc(Box::new(None)),
                EditorKind::Level => self.state = EditorState::Level(Box::new(None)),
            },
            Message::EditAttack(id) => {
                self.state = EditorState::Attack(Box::new(attack::item::Page::load_by_id(id)))
            }
            Message::Attack(message) => {
                if let EditorState::Attack(attack) = &mut self.state {
                    attack.update(message);
                }
            }
            Message::AttackList(message) => {
                if let EditorState::AttackList(page) = &mut self.state {
                    if let Some(new_message) = page.update(message) {
                        return self.update(new_message);
                    }
                }
            }
            Message::Npc(message) => {
                if let EditorState::Npc(npc) = &mut self.state {
                    npc::update(npc, message);
                }
            }
            Message::Level(message) => {
                if let EditorState::Level(level) = &mut self.state {
                    level::update(level, message);
                }
            }
        }
        Task::none()
    }
    fn view(&self, _window: window::Id) -> Element<Message> {
        let editor_kind_picker = row![
            text("Editor kind"),
            pick_list(
                EditorKind::items(),
                self.selected_kind(),
                Message::SelectKind
            )
            .placeholder("Editor kind")
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
            EditorState::AttackList(page) => {
                let element = page.view().map(Message::AttackList);
                contents = contents.push(element);
            }
            EditorState::Npc(npc) => {
                let element = npc::view(npc).map(Message::Npc);
                contents = contents.push(element);
            }
            EditorState::Level(level) => {
                let element = level::view(level).map(Message::Level);
                contents = contents.push(element);
            }
        }

        column![
            editor_kind_picker,
            container(contents)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .align_x(Alignment::Center)
        .into()
    }
    fn theme(&self, _window: window::Id) -> Theme {
        Theme::TokyoNight
    }
}

impl App {
    fn selected_kind(&self) -> Option<EditorKind> {
        match self.state {
            EditorState::NotSelected => None,
            EditorState::Attack(_) => Some(EditorKind::Attack),
            EditorState::AttackList(_) => Some(EditorKind::AttackList),
            EditorState::Npc(_) => Some(EditorKind::Npc),
            EditorState::Level(_) => Some(EditorKind::Level),
        }
    }
}
