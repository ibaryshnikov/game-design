use iced::widget::{Row, Scrollable, button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element};

use shared::npc::{NpcAttackInfo, NpcConstructor};

use super::get_item_file_path;
use crate::attack::list::{AttackInfo, load_available_attack_list};

pub struct Page {
    id: u32,
    data: NpcConstructor,
    selected_attack: Option<NpcAttackInfo>,
    available_attack_list: Vec<NpcAttackInfo>,
}

impl Page {
    pub fn load_by_id(id: u32) -> Self {
        let available_attack_list = make_picker_items(load_available_attack_list());
        Page {
            id,
            data: load_by_id(id),
            selected_attack: None,
            available_attack_list,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    WriteFile,
    ChangeRespawnTime(String),
    SelectAttack(NpcAttackInfo),
    AddAttack(NpcAttackInfo),
    RemoveAttack(usize),
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

fn make_picker_items(npc_list: Vec<AttackInfo>) -> Vec<NpcAttackInfo> {
    npc_list
        .into_iter()
        .filter(|item| item.status.is_active())
        .map(|AttackInfo { id, name, .. }| NpcAttackInfo { id, name })
        .collect()
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ReadFile => {
                self.data = load_by_id(self.id);
            }
            Message::WriteFile => save_by_id(&self.data, self.id),
            Message::ChangeRespawnTime(value) => {
                let parsed = value.parse::<u128>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.respawn_time = parsed;
            }
            Message::SelectAttack(attack) => {
                self.selected_attack = Some(attack);
            }
            Message::AddAttack(attack) => {
                self.data.attacks.push(attack);
            }
            Message::RemoveAttack(index) => {
                if index >= self.data.attacks.len() {
                    return;
                }
                self.data.attacks.remove(index);
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

        let mut attack_list = column![].align_x(Alignment::Center).spacing(10);
        for (index, attack_id) in self.data.attacks.iter().enumerate() {
            let attack_row = row![
                text(format!("Attack id: {attack_id}")),
                button("delete").on_press(Message::RemoveAttack(index)),
            ]
            .spacing(10);
            attack_list = attack_list.push(attack_row);
        }
        let message_add_attack = self.selected_attack.clone().map(Message::AddAttack);
        let add_attack_row = row![
            pick_list(
                &self.available_attack_list[..],
                self.selected_attack.clone(),
                Message::SelectAttack
            ),
            button("add").on_press_maybe(message_add_attack),
        ]
        .spacing(10);

        let npc_details_column = column![
            editor_row(
                "Respawn time, s",
                text_input("Respawn time, s", &format!("{}", self.data.respawn_time))
                    .on_input(Message::ChangeRespawnTime),
            ),
            text("Add attack:"),
            add_attack_row,
            text("Attacks:"),
            attack_list,
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let scrollable_details = Scrollable::new(npc_details_column);
        let npc_details = container(scrollable_details).width(500);
        contents = contents.push(npc_details);

        contents.into()
    }
}

fn editor_row<'a, T: Into<Element<'a, Message>>>(label: &'a str, element: T) -> Row<'a, Message> {
    row![text(label), element.into()]
        .align_y(Alignment::Center)
        .spacing(10)
}
