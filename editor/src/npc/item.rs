use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element};

use shared::npc::NpcConstructor;

use super::get_item_file_path;

pub struct Page {
    id: u32,
    data: NpcConstructor,
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
    ChangeCloseMeleeAttackDistance(String),
    ChangeMeleeAttackDistance(String),
    ChangeRangedAttackDistance(String),
}

fn read_file() -> Option<NpcConstructor> {
    let contents = std::fs::read("data/npc.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(npc: &Option<NpcConstructor>) {
    let Some(npc) = npc else { return };
    let contents = serde_json::to_vec_pretty(npc).expect("Should encode NpcConstructor");
    std::fs::write("data/npc.json", contents).expect("Should write NpcConstructor to a file");
}

fn load_by_id(id: u32) -> NpcConstructor {
    let file_path = get_item_file_path(id);
    let contents = std::fs::read(file_path).expect("Should read NpcConstructor from a file");
    serde_json::from_slice(&contents).expect("Should decode NpcConstructor")
}

pub fn save_by_id(npc: &NpcConstructor, id: u32) {
    let file_path = get_item_file_path(id);
    let contents = serde_json::to_vec_pretty(npc).expect("Should encode NpcConstructor");
    std::fs::write(file_path, contents).expect("Should write NpcConstructor to a file");
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ReadFile => {
                self.data = load_by_id(self.id);
            }
            Message::WriteFile => save_by_id(&self.data, self.id),
            Message::ChangeCloseMeleeAttackDistance(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.close_melee_attack_distance = parsed;
            }
            Message::ChangeMeleeAttackDistance(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.melee_attack_distance = parsed;
            }
            Message::ChangeRangedAttackDistance(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.ranged_attack_distance = parsed;
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

        let npc_details_column = column![
            row![
                text("Close melee attack distance"),
                text_input(
                    "Close melee attack distance",
                    &format!("{}", self.data.close_melee_attack_distance)
                )
                .on_input(Message::ChangeCloseMeleeAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Melee attack distance"),
                text_input(
                    "Melee attack distance",
                    &format!("{}", self.data.melee_attack_distance)
                )
                .on_input(Message::ChangeMeleeAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Ranged attack distance"),
                text_input(
                    "Ranged attack distance",
                    &format!("{}", self.data.ranged_attack_distance)
                )
                .on_input(Message::ChangeRangedAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let npc_details = container(npc_details_column).width(300);
        contents = contents.push(npc_details);

        contents.into()
    }
}
