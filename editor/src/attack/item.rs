use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element};

use shared::attack::{AttackConstructor, AttackKind, AttackOrder};

use super::get_item_file_path;
use crate::common::editor_row;

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
    ChangeName(String),
    ChangeDelay(String),
    ChangeTimeToComplete(String),
    ChangeAftercast(String),
    ChangeOrder(AttackOrder),
    ChangeRangeFrom(String),
    ChangeRangeTo(String),
    ChangeWidthAngle(String),
    ChangeKind(AttackKind),
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

pub(super) fn delete_file_by_id(id: u32) {
    let file_path = get_item_file_path(id);
    if let Err(e) = std::fs::remove_file(file_path) {
        println!("Error removing file for attack {id}: {e}");
    }
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ReadFile => {
                self.data = load_by_id(self.id);
            }
            Message::WriteFile => {
                save_by_id(&self.data, self.id);
                super::list::update_name_for(self.id, self.data.name.clone());
            }
            Message::ChangeName(value) => {
                self.data.name = value;
            }
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

    pub fn view(&self) -> Element<'_, Message> {
        let mut contents = column![
            row![
                button("Reload from disk").on_press(Message::ReadFile),
                button("Save").on_press(Message::WriteFile),
            ]
            .spacing(10)
        ]
        .align_x(Alignment::Center)
        .spacing(10);

        let attack_details_column = column![
            text(format!("Id {}", self.id)),
            editor_row(
                "Name",
                text_input("Attack name", &self.data.name).on_input(Message::ChangeName)
            ),
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
                text_input(
                    "Attack width angle",
                    &format!("{:?}", self.data.width_angle)
                )
                .on_input(Message::ChangeWidthAngle),
            ),
            editor_row(
                "Kind",
                pick_list(
                    AttackKind::options(),
                    Some(self.data.kind.clone()),
                    Message::ChangeKind
                )
                .placeholder("Attack kind")
            ),
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let attack_details = container(attack_details_column).width(300);
        contents = contents.push(attack_details);

        contents.into()
    }
}
