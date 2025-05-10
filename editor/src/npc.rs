use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element};

use shared::npc::NpcConstructor;

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

fn write_file(attack: &Option<NpcConstructor>) {
    let Some(attack) = attack else { return };
    let contents = serde_json::to_vec(attack).expect("Should encode NpcConstructor");
    std::fs::write("data/npc.json", contents).expect("Should write NpcConstructor to a file");
}

pub fn update(npc: &mut Option<NpcConstructor>, message: Message) {
    match message {
        Message::ReadFile => {
            let contents = read_file();
            if contents.is_some() {
                *npc = contents;
            } else {
                *npc = Some(NpcConstructor::default())
            }
        }
        Message::WriteFile => write_file(npc),
        Message::ChangeCloseMeleeAttackDistance(value) => {
            let parsed = value.parse::<f32>().ok();
            let Some(parsed) = parsed else {
                return;
            };
            if let Some(npc) = npc {
                npc.close_melee_attack_distance = parsed;
            }
        }
        Message::ChangeMeleeAttackDistance(value) => {
            let parsed = value.parse::<f32>().ok();
            let Some(parsed) = parsed else {
                return;
            };
            if let Some(npc) = npc {
                npc.melee_attack_distance = parsed;
            }
        }
        Message::ChangeRangedAttackDistance(value) => {
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

pub fn view(npc: &Option<NpcConstructor>) -> Element<Message> {
    let mut contents = column![
        button("Read").on_press(Message::ReadFile),
        button("Write").on_press(Message::WriteFile),
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
                .on_input(Message::ChangeCloseMeleeAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Melee attack distance"),
                text_input(
                    "Melee attack distance",
                    &format!("{}", npc.melee_attack_distance)
                )
                .on_input(Message::ChangeMeleeAttackDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Ranged attack distance"),
                text_input(
                    "Ranged attack distance",
                    &format!("{}", npc.ranged_attack_distance)
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
    }

    contents.into()
}
