use iced::widget::{
    Container, button, checkbox, column, container, horizontal_space, row, text, vertical_rule,
};
use iced::{Alignment, Element, Length};
use serde::{Deserialize, Serialize};

use shared::attack::AttackConstructor;
use shared::list::EntryStatus;

use super::FOLDER_PATH;
use crate::utils::combine;

const FILE_NAME: &str = "list.json";
const FILE_PATH: &str = combine!(FOLDER_PATH, FILE_NAME);

pub struct Page {
    data: AttackList,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct AttackList {
    last_id: u32,
    list: Vec<AttackInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttackInfo {
    pub id: u32,
    pub name: String,
    pub status: EntryStatus,
}

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    ToggleEntryStatus(u32, bool),
    DeleteEntry(u32),
    CreateNew,
    Edit(u32),
}

fn read_file() -> Option<AttackList> {
    let contents = std::fs::read(FILE_PATH).ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(attack_list: &AttackList) {
    let contents = serde_json::to_vec_pretty(attack_list).expect("Should encode AttackList");
    std::fs::write(FILE_PATH, contents).expect("Should write AttackList to a file");
}

fn find_entry_mut(list: &mut [AttackInfo], id: u32) -> Option<&mut AttackInfo> {
    list.iter_mut().find(|item| item.id == id)
}

pub fn load_available_attack_list() -> Vec<AttackInfo> {
    read_file().map(|data| data.list).unwrap_or_default()
}

fn show_entry(attack_list: &mut AttackList, id: u32) {
    let Some(entry) = find_entry_mut(&mut attack_list.list, id) else {
        return;
    };
    if let EntryStatus::Active = entry.status {
        return;
    };
    entry.status = EntryStatus::Active;
    write_file(attack_list);
}

fn hide_entry(attack_list: &mut AttackList, id: u32) {
    let Some(entry) = find_entry_mut(&mut attack_list.list, id) else {
        return;
    };
    if let EntryStatus::Hidden = entry.status {
        return;
    };
    entry.status = EntryStatus::Hidden;
    write_file(attack_list);
}

fn delete_entry(data: &mut AttackList, id: u32) {
    if let Some(index) = data.list.iter().position(|entry| entry.id == id) {
        let attack = data.list.remove(index);
        super::item::delete_file_by_id(attack.id);
    }
    let max_id = data
        .list
        .iter()
        .map(|entry| entry.id)
        .max()
        .unwrap_or_default();
    data.last_id = max_id;
    write_file(data);
}

fn load_data() -> AttackList {
    read_file().unwrap_or_default()
}

pub(super) fn update_name_for(id: u32, name: String) {
    let mut data = load_data();
    for item in data.list.iter_mut() {
        if item.id == id {
            item.name = name;
            break;
        }
    }
    write_file(&data);
}

impl Page {
    pub fn load() -> Self {
        Self { data: load_data() }
    }
    pub fn update(&mut self, message: Message) -> Option<super::Message> {
        match message {
            Message::ReadFile => {
                self.data = load_data();
            }
            Message::ToggleEntryStatus(id, is_active) => {
                if is_active {
                    show_entry(&mut self.data, id);
                } else {
                    hide_entry(&mut self.data, id);
                }
            }
            Message::DeleteEntry(id) => delete_entry(&mut self.data, id),
            Message::CreateNew => {
                self.data.last_id += 1;
                let id = self.data.last_id;
                let name = String::new();
                let new_attack = AttackConstructor::new(name.clone());
                super::item::save_by_id(&new_attack, id);
                let new_entry = AttackInfo {
                    id,
                    name,
                    status: EntryStatus::Active,
                };
                self.data.list.push(new_entry);
                write_file(&self.data);
                return Some(super::Message::EditItem(id));
            }
            Message::Edit(id) => {
                return Some(super::Message::EditItem(id));
            }
        }
        None
    }
    pub fn view(&self) -> Element<'_, Message> {
        let heading_row = row![
            text(format!("Last item id: {}", self.data.last_id)),
            horizontal_space(),
            button("Refresh").on_press(Message::ReadFile),
            button("Create new").on_press(Message::CreateNew),
        ]
        .spacing(10);
        let mut details_column = column![heading_row].align_x(Alignment::Start).spacing(10);
        for item in self.data.list.iter() {
            let id = item.id;
            let item_row = row![
                text(format!("{}", item.id))
                    .width(portion(1))
                    .align_x(Alignment::Center),
                make_rule(1, Alignment::Start),
                text(&item.name).width(portion(5)),
                make_rule(1, Alignment::End),
                checkbox("", item.status.is_active())
                    .on_toggle(move |value| Message::ToggleEntryStatus(id, value)),
                button(text("Edit").align_x(Alignment::Center))
                    .on_press(Message::Edit(item.id))
                    .width(portion(3)),
                button(text("Delete").align_x(Alignment::Center))
                    .on_press(Message::DeleteEntry(item.id))
                    .width(portion(3)),
            ]
            .spacing(5)
            .padding([0, 5])
            .align_y(Alignment::Center)
            .height(Length::Shrink);
            let item_row = container(item_row).style(container::bordered_box);
            details_column = details_column.push(item_row);
        }
        container(details_column).width(400).into()
    }
}

fn portion(value: u16) -> Length {
    Length::FillPortion(value)
}

fn make_rule(part: u16, alignment: Alignment) -> Container<'static, Message> {
    container(vertical_rule(5))
        .align_x(alignment)
        .width(portion(part))
        .height(23)
}
