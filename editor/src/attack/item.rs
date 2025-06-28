use iced::widget::{Row, button, column, container, pick_list, row, text, text_input};
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
    ChangeTimeToComplete(String),
    ChangeAftercast(String),
    ChangeOrder(AttackOrder),
    ChangeRangeFrom(String),
    ChangeRangeTo(String),
    ChangeWidthAngle(String),
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
            Message::ChangeTimeToComplete(value) => {
                let parsed = value.parse::<u128>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.time_to_complete = parsed;
            }
            Message::ChangeAftercast(value) => {
                let parsed = value.parse::<u128>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.aftercast = parsed;
            }
            Message::ChangeOrder(order) => {
                self.data.order = order;
            }
            Message::ChangeRangeFrom(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.range.from = parsed;
            }
            Message::ChangeRangeTo(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.range.to = parsed;
            }
            Message::ChangeWidthAngle(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.width_angle = parsed;
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
            editor_row(
                "Delay",
                text_input("Attack delay", &format!("{}", self.data.delay))
                    .on_input(Message::ChangeDelay),
            ),
            editor_row(
                "Time to complete",
                text_input(
                    "Attack time to complete",
                    &format!("{}", self.data.time_to_complete)
                )
                .on_input(Message::ChangeTimeToComplete),
            ),
            editor_row(
                "Aftercast",
                text_input("Attack aftercast", &format!("{}", self.data.aftercast))
                    .on_input(Message::ChangeAftercast),
            ),
            editor_row(
                "Order",
                pick_list(
                    AttackOrder::options(),
                    Some(self.data.order.clone()),
                    Message::ChangeOrder
                )
                .placeholder("Attack order"),
            ),
            editor_row(
                "Range from",
                text_input("Attack range from", &format!("{}", self.data.range.from))
                    .on_input(Message::ChangeRangeFrom),
            ),
            editor_row(
                "Range to",
                text_input("Attack range to", &format!("{}", self.data.range.to))
                    .on_input(Message::ChangeRangeTo),
            ),
            editor_row(
                "Width angle",
                text_input("Attack width angle", &format!("{}", self.data.width_angle))
                    .on_input(Message::ChangeWidthAngle),
            ),
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

fn editor_row<'a, T: Into<Element<'a, Message>>>(label: &'a str, element: T) -> Row<'a, Message> {
    row![text(label), element.into()]
        .align_y(Alignment::Center)
        .spacing(10)
}
