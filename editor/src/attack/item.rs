use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element};

use shared::attack::{AttackConstructor, AttackKind, AttackOrder};

use super::get_item_file_path;

pub struct Page {
    id: u32,
    data: AttackConstructor,
}

impl Page {
    pub fn load_by_id(id: u32) -> Self {
        Page {
            id,
            data: load_by_id(id),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    WriteFile,
    ChangeDelay(String),
    ChangeOrder(AttackOrder),
    ChangeKind(AttackKind),
}

fn read_file() -> Option<AttackConstructor> {
    let contents = std::fs::read("data/attack.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(attack: &Option<AttackConstructor>) {
    let Some(attack) = attack else { return };
    let contents = serde_json::to_vec_pretty(attack).expect("Should encode AttackConstructor");
    std::fs::write("data/attack.json", contents).expect("Should write AttackConstructor to a file");
}

fn load_by_id(id: u32) -> AttackConstructor {
    let file_path = get_item_file_path(id);
    let contents = std::fs::read(file_path).expect("Should read AttackConstructor from a file");
    serde_json::from_slice(&contents).expect("Should decode AttackConstructor")
}

pub fn save_by_id(attack: &AttackConstructor, id: u32) {
    let file_path = get_item_file_path(id);
    let contents = serde_json::to_vec_pretty(attack).expect("Should encode AttackConstructor");
    std::fs::write(file_path, contents).expect("Should write AttackConstructor to a file");
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ReadFile => {
                self.data = load_by_id(self.id);
            }
            Message::WriteFile => save_by_id(&self.data, self.id),
            Message::ChangeDelay(value) => {
                let parsed = value.parse::<u128>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.delay = parsed;
            }
            Message::ChangeOrder(order) => {
                self.data.order = order;
            }
            Message::ChangeKind(kind) => {
                self.data.kind = kind;
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mut contents = column![
            button("Reload from disk").on_press(Message::ReadFile),
            button("Save").on_press(Message::WriteFile),
        ]
        .align_x(Alignment::Center)
        .spacing(10);

        let attack_details_column = column![
            row![text("Name:"), text(&self.data.name)].spacing(10),
            row![
                text("Delay"),
                text_input("Attack delay", &format!("{}", self.data.delay))
                    .on_input(Message::ChangeDelay),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Order"),
                pick_list(
                    AttackOrder::options(),
                    Some(self.data.order.clone()),
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
                    Some(self.data.kind.clone()),
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

        contents.into()
    }
}
