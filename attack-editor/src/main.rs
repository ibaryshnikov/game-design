use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{window, Alignment, Element, Length, Renderer, Settings, Task, Theme};
use iced_widget::graphics::{self, compositor};
use iced_winit::Program;

use serde::{Deserialize, Serialize};

use shared::attack::{AttackConstructor, AttackKind, AttackOrder};

fn main() {
    let app = App { attack: None };
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
    ReadFile,
    WriteFile,
    ChangeDelay(String),
    ChangeOrder(AttackOrder),
    ChangeKind(AttackKind),
}

struct App {
    attack: Option<AttackConstructor>,
}

fn read_file() -> Option<AttackConstructor> {
    let contents = std::fs::read("data/attack.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(attack: &Option<AttackConstructor>) {
    let Some(attack) = attack else { return };
    let contents = serde_json::to_vec(attack).expect("Should encode a AttackConstructor");
    std::fs::write("data/attack.json", contents).expect("Should write AttackConstructor to a file");
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
            Message::ReadFile => {
                let contents = read_file();
                if contents.is_some() {
                    self.attack = contents;
                } else {
                    self.attack = Some(AttackConstructor::default())
                }
            }
            Message::WriteFile => write_file(&self.attack),
            Message::ChangeDelay(value) => {
                let parsed = value.parse::<u128>().ok();
                let Some(parsed) = parsed else {
                    return Task::none();
                };
                if let Some(attack) = &mut self.attack {
                    attack.delay = parsed;
                }
            }
            Message::ChangeOrder(order) => {
                if let Some(attack) = &mut self.attack {
                    attack.order = order;
                }
            }
            Message::ChangeKind(kind) => {
                if let Some(attack) = &mut self.attack {
                    attack.kind = kind;
                }
            }
        }
        Task::none()
    }
    fn view(&self, _window: window::Id) -> Element<Message> {
        let mut contents = column![
            button("Read").on_press(Message::ReadFile),
            button("Write").on_press(Message::WriteFile),
        ]
        .align_x(Alignment::Center)
        .spacing(10);

        if let Some(attack) = &self.attack {
            let attack_details_column = column![
                row![
                    text("Delay"),
                    text_input("Attack delay", &format!("{}", attack.delay))
                        .on_input(Message::ChangeDelay),
                ]
                .align_y(Alignment::Center)
                .spacing(10),
                row![
                    text("Order"),
                    pick_list(
                        AttackOrder::options(),
                        Some(attack.order.clone()),
                        Message::ChangeOrder
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
                        Message::ChangeKind
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

        container(contents)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
    fn theme(&self, _window: window::Id) -> Theme {
        Theme::TokyoNight
    }
}
