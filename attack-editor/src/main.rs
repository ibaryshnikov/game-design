use std::fmt::{self, Display};

use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{window, Alignment, Element, Length, Renderer, Settings, Task, Theme};
use iced_widget::graphics::{self, compositor};
use iced_winit::Program;

use shared::attack::{AttackConstructor, AttackKind, AttackOrder};
use shared::npc::NpcConstructor;

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
enum AttackMessage {
    ReadFile,
    WriteFile,
    ChangeDelay(String),
    ChangeOrder(AttackOrder),
    ChangeKind(AttackKind),
}

#[derive(Debug, Clone)]
enum NpcMessage {
    ReadFile,
    WriteFile,
    ChangeCloseMeleeAttackDistance(String),
    ChangeMeleeAttackDistance(String),
    ChangeRangedAttackDistance(String),
}

#[derive(Debug, Clone)]
enum Message {
    SelectKind(EditorKind),
    Attack(AttackMessage),
    Npc(NpcMessage),
}

#[derive(Debug, Clone, PartialEq)]
enum EditorKind {
    Attack,
    Npc,
}

impl Display for EditorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
enum EditorState {
    NotSelected,
    Attack(Box<Option<AttackConstructor>>),
    Npc(Box<Option<NpcConstructor>>),
}

struct App {
    state: EditorState,
}

fn read_file_attack() -> Option<AttackConstructor> {
    let contents = std::fs::read("data/attack.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file_attack(attack: &Option<AttackConstructor>) {
    let Some(attack) = attack else { return };
    let contents = serde_json::to_vec(attack).expect("Should encode AttackConstructor");
    std::fs::write("data/attack.json", contents).expect("Should write AttackConstructor to a file");
}

fn update_attack(attack: &mut Option<AttackConstructor>, message: AttackMessage) {
    match message {
        AttackMessage::ReadFile => {
            let contents = read_file_attack();
            if contents.is_some() {
                *attack = contents;
            } else {
                *attack = Some(AttackConstructor::default())
            }
        }
        AttackMessage::WriteFile => write_file_attack(attack),
        AttackMessage::ChangeDelay(value) => {
            let parsed = value.parse::<u128>().ok();
            let Some(parsed) = parsed else {
                return;
            };
            if let Some(attack) = attack {
                attack.delay = parsed;
            }
        }
        AttackMessage::ChangeOrder(order) => {
            if let Some(attack) = attack {
                attack.order = order;
            }
        }
        AttackMessage::ChangeKind(kind) => {
            if let Some(attack) = attack {
                attack.kind = kind;
            }
        }
    }
}

fn view_attack(attack: &Option<AttackConstructor>) -> Element<AttackMessage> {
    let mut contents = column![
        button("Read").on_press(AttackMessage::ReadFile),
        button("Write").on_press(AttackMessage::WriteFile),
    ]
    .align_x(Alignment::Center)
    .spacing(10);

    if let Some(attack) = attack {
        let attack_details_column = column![
            row![
                text("Delay"),
                text_input("Attack delay", &format!("{}", attack.delay))
                    .on_input(AttackMessage::ChangeDelay),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Order"),
                pick_list(
                    AttackOrder::options(),
                    Some(attack.order.clone()),
                    AttackMessage::ChangeOrder
                )
                .placeholder("Attack order"),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Kind"),
                pick_list(
                    AttackKind::options(),
                    Some(attack.kind.clone()),
                    AttackMessage::ChangeKind
                )
                .placeholder("Attack kind")
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let attack_details = container(attack_details_column).width(300);
        contents = contents.push(attack_details);
    }

    contents.into()
}

fn read_file_npc() -> Option<NpcConstructor> {
    let contents = std::fs::read("data/npc.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file_npc(attack: &Option<NpcConstructor>) {
    let Some(attack) = attack else { return };
    let contents = serde_json::to_vec(attack).expect("Should encode NpcConstructor");
    std::fs::write("data/npc.json", contents).expect("Should write NpcConstructor to a file");
}

fn update_npc(npc: &mut Option<NpcConstructor>, message: NpcMessage) {
    match message {
        NpcMessage::ReadFile => {
            let contents = read_file_npc();
            if contents.is_some() {
                *npc = contents;
            } else {
                *npc = Some(NpcConstructor::default())
            }
        }
        NpcMessage::WriteFile => write_file_npc(npc),
        NpcMessage::ChangeCloseMeleeAttackDistance(value) => {
            let parsed = value.parse::<f32>().ok();
            let Some(parsed) = parsed else {
                return;
            };
            if let Some(npc) = npc {
                npc.close_melee_attack_distance = parsed;
            }
        }
        NpcMessage::ChangeMeleeAttackDistance(value) => {
            let parsed = value.parse::<f32>().ok();
            let Some(parsed) = parsed else {
                return;
            };
            if let Some(npc) = npc {
                npc.melee_attack_distance = parsed;
            }
        }
        NpcMessage::ChangeRangedAttackDistance(value) => {
            let parsed = value.parse::<f32>().ok();
            let Some(parsed) = parsed else {
                return;
            };
            if let Some(npc) = npc {
                npc.ranged_attack_distance = parsed;
            }
        }
    }
}

fn view_npc(npc: &Option<NpcConstructor>) -> Element<NpcMessage> {
    let mut contents = column![
        button("Read").on_press(NpcMessage::ReadFile),
        button("Write").on_press(NpcMessage::WriteFile),
    ]
    .align_x(Alignment::Center)
    .spacing(10);

    if let Some(npc) = npc {
        let npc_details_column = column![
            row![
                text("Close melee attack distance"),
                text_input(
                    "Close melee attack distance",
                    &format!("{}", npc.close_melee_attack_distance)
                )
                .on_input(NpcMessage::ChangeCloseMeleeAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Melee attack distance"),
                text_input(
                    "Melee attack distance",
                    &format!("{}", npc.melee_attack_distance)
                )
                .on_input(NpcMessage::ChangeMeleeAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Ranged attack distance"),
                text_input(
                    "Ranged attack distance",
                    &format!("{}", npc.ranged_attack_distance)
                )
                .on_input(NpcMessage::ChangeRangedAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let npc_details = container(npc_details_column).width(300);
        contents = contents.push(npc_details);
    }

    contents.into()
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
        "Attack editor".into()
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectKind(kind) => match kind {
                EditorKind::Attack => self.state = EditorState::Attack(Box::new(None)),
                EditorKind::Npc => self.state = EditorState::Npc(Box::new(None)),
            },
            Message::Attack(message) => {
                if let EditorState::Attack(attack) = &mut self.state {
                    update_attack(attack, message);
                }
            }
            Message::Npc(message) => {
                if let EditorState::Npc(npc) = &mut self.state {
                    update_npc(npc, message);
                }
            }
        }
        Task::none()
    }
    fn view(&self, _window: window::Id) -> Element<Message> {
        let selected = match self.state {
            EditorState::NotSelected => None,
            EditorState::Attack(_) => Some(EditorKind::Attack),
            EditorState::Npc(_) => Some(EditorKind::Npc),
        };
        let editor_kind_picker = row![
            text("Editor kind"),
            pick_list(
                [EditorKind::Attack, EditorKind::Npc],
                selected,
                Message::SelectKind
            )
            .placeholder("Editor kind")
        ]
        .align_y(Alignment::Center)
        .spacing(10);
        let mut contents = column![].align_x(Alignment::Center).spacing(10);

        match &self.state {
            EditorState::NotSelected => (),
            EditorState::Attack(attack) => {
                let element = view_attack(attack).map(Message::Attack);
                contents = contents.push(element);
            }
            EditorState::Npc(npc) => {
                let element = view_npc(npc).map(Message::Npc);
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
