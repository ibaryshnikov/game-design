use iced::widget::{button, column, container, pick_list, row, text, text_input, Scrollable};
use iced::{Alignment, Element};

use shared::npc::{NpcAttackInfo, NpcConstructor};

use super::get_item_file_path;
use crate::attack::list::{load_available_attack_list, AttackInfo};

pub struct Page {
    id: u32,
    data: NpcConstructor,
    selected_close_melee_attack: Option<NpcAttackInfo>,
    selected_melee_attack: Option<NpcAttackInfo>,
    selected_ranged_attack: Option<NpcAttackInfo>,
    available_attack_list: Vec<NpcAttackInfo>,
}

impl Page {
    pub fn load_by_id(id: u32) -> Self {
        let available_attack_list = make_picker_items(load_available_attack_list());
        Page {
            id,
            data: load_by_id(id),
            selected_close_melee_attack: None,
            selected_melee_attack: None,
            selected_ranged_attack: None,
            available_attack_list,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    WriteFile,
    ChangeCloseMeleeAttackDistance(String),
    SelectCloseMeleeAttack(NpcAttackInfo),
    AddCloseMeleeAttack(NpcAttackInfo),
    RemoveCloseMeleeAttack(usize),
    ChangeMeleeAttackDistance(String),
    SelectMeleeAttack(NpcAttackInfo),
    AddMeleeAttack(NpcAttackInfo),
    RemoveMeleeAttack(usize),
    ChangeRangedAttackDistance(String),
    SelectRangedAttack(NpcAttackInfo),
    AddRangedAttack(NpcAttackInfo),
    RemoveRangedAttack(usize),
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
            Message::ChangeCloseMeleeAttackDistance(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.close_melee_attack_distance = parsed;
            }
            Message::SelectCloseMeleeAttack(attack) => {
                self.selected_close_melee_attack = Some(attack);
            }
            Message::AddCloseMeleeAttack(attack) => {
                self.data.close_melee_attacks.push(attack);
            }
            Message::RemoveCloseMeleeAttack(index) => {
                if index >= self.data.close_melee_attacks.len() {
                    return;
                }
                self.data.close_melee_attacks.remove(index);
            }
            Message::ChangeMeleeAttackDistance(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.melee_attack_distance = parsed;
            }
            Message::SelectMeleeAttack(attack) => {
                self.selected_melee_attack = Some(attack);
            }
            Message::AddMeleeAttack(attack) => {
                self.data.melee_attacks.push(attack);
            }
            Message::RemoveMeleeAttack(index) => {
                if index >= self.data.melee_attacks.len() {
                    return;
                }
                self.data.melee_attacks.remove(index);
            }
            Message::ChangeRangedAttackDistance(value) => {
                let parsed = value.parse::<f32>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.data.ranged_attack_distance = parsed;
            }
            Message::SelectRangedAttack(attack) => {
                self.selected_ranged_attack = Some(attack);
            }
            Message::AddRangedAttack(attack) => {
                self.data.ranged_attacks.push(attack);
            }
            Message::RemoveRangedAttack(index) => {
                if index >= self.data.ranged_attacks.len() {
                    return;
                }
                self.data.ranged_attacks.remove(index);
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

        let mut close_melee_attack_list = column![].align_x(Alignment::Center).spacing(10);
        for (index, attack_id) in self.data.close_melee_attacks.iter().enumerate() {
            let attack_row = row![
                text(format!("Attack id: {}", attack_id)),
                button("delete").on_press(Message::RemoveCloseMeleeAttack(index)),
            ]
            .spacing(10);
            close_melee_attack_list = close_melee_attack_list.push(attack_row);
        }
        let message_add_close_melee_attack = self
            .selected_close_melee_attack
            .clone()
            .map(Message::AddCloseMeleeAttack);
        let add_close_melee_attack_row = row![
            pick_list(
                &self.available_attack_list[..],
                self.selected_close_melee_attack.clone(),
                Message::SelectCloseMeleeAttack
            ),
            button("add").on_press_maybe(message_add_close_melee_attack),
        ]
        .spacing(10);

        let mut melee_attack_list = column![].align_x(Alignment::Center).spacing(10);
        for (index, attack_id) in self.data.melee_attacks.iter().enumerate() {
            let attack_row = row![
                text(format!("Attack id: {}", attack_id)),
                button("delete").on_press(Message::RemoveMeleeAttack(index)),
            ]
            .spacing(10);
            melee_attack_list = melee_attack_list.push(attack_row);
        }
        let message_add_melee_attack = self
            .selected_melee_attack
            .clone()
            .map(Message::AddMeleeAttack);
        let add_melee_attack_row = row![
            pick_list(
                &self.available_attack_list[..],
                self.selected_melee_attack.clone(),
                Message::SelectMeleeAttack
            ),
            button("add").on_press_maybe(message_add_melee_attack),
        ]
        .spacing(10);

        let mut ranged_attack_list = column![].align_x(Alignment::Center).spacing(10);
        for (index, attack_id) in self.data.ranged_attacks.iter().enumerate() {
            let attack_row = row![
                text(format!("Attack id: {}", attack_id)),
                button("delete").on_press(Message::RemoveRangedAttack(index)),
            ]
            .spacing(10);
            ranged_attack_list = ranged_attack_list.push(attack_row);
        }
        let message_add_ranged_attack = self
            .selected_ranged_attack
            .clone()
            .map(Message::AddRangedAttack);
        let add_ranged_attack_row = row![
            pick_list(
                &self.available_attack_list[..],
                self.selected_ranged_attack.clone(),
                Message::SelectRangedAttack
            ),
            button("add").on_press_maybe(message_add_ranged_attack),
        ]
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
            text("Add close melee attack:"),
            add_close_melee_attack_row,
            text("Close melee attacks:"),
            close_melee_attack_list,
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
            text("Add melee attack:"),
            add_melee_attack_row,
            text("Melee attacks:"),
            melee_attack_list,
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
            text("Add ranged attack:"),
            add_ranged_attack_row,
            text("Ranged attacks:"),
            ranged_attack_list,
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let scrollable_details = Scrollable::new(npc_details_column);
        let npc_details = container(scrollable_details).width(500);
        contents = contents.push(npc_details);

        contents.into()
    }
}
