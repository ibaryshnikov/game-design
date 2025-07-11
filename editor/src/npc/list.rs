use iced::widget::{
    Container, button, column, container, horizontal_space, row, text, text_input, vertical_rule,
};
use iced::{Alignment, Element, Length};
use serde::{Deserialize, Serialize};

use shared::list::EntryStatus;
use shared::npc::NpcConstructor;

use super::FOLDER_PATH;
use crate::utils::combine;

const FILE_NAME: &str = "list.json";
const FILE_PATH: &str = combine!(FOLDER_PATH, FILE_NAME);

pub struct Page {
    data: NpclList,
    new_entry_name: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NpclList {
    last_id: u32,
    list: Vec<NpcInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NpcInfo {
    pub id: u32,
    pub name: String,
    pub status: EntryStatus,
}

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    ShowEntry(u32),
    HideEntry(u32),
    ChangeNewEntryName(String),
    CreateNew,
    Edit(u32),
}

fn read_file() -> Option<NpclList> {
    let contents = std::fs::read(FILE_PATH).ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(npc_list: &NpclList) {
    let contents = serde_json::to_vec_pretty(npc_list).expect("Should encode NpclList");
    std::fs::write(FILE_PATH, contents).expect("Should write NpclList to a file");
}

fn find_entry_mut(list: &mut [NpcInfo], id: u32) -> Option<&mut NpcInfo> {
    list.iter_mut().find(|item| item.id == id)
}

pub fn load_available_npc_list() -> Vec<NpcInfo> {
    read_file().map(|data| data.list).unwrap_or_default()
}

fn show_entry(level_list: &mut NpclList, id: u32) {
    let Some(entry) = find_entry_mut(&mut level_list.list, id) else {
        return;
    };
    if let EntryStatus::Active = entry.status {
        return;
    };
    entry.status = EntryStatus::Active;
    write_file(level_list);
}

fn hide_entry(level_list: &mut NpclList, id: u32) {
    let Some(entry) = find_entry_mut(&mut level_list.list, id) else {
        return;
    };
    if let EntryStatus::Hidden = entry.status {
        return;
    };
    entry.status = EntryStatus::Hidden;
    write_file(level_list);
}

fn load_data() -> NpclList {
    read_file().unwrap_or_default()
}

impl Page {
    pub fn load() -> Self {
        Self {
            data: load_data(),
            new_entry_name: String::new(),
        }
    }
    pub fn update(&mut self, message: Message) -> Option<super::Message> {
        match message {
            Message::ReadFile => {
                self.data = load_data();
            }
            Message::ShowEntry(id) => show_entry(&mut self.data, id),
            Message::HideEntry(id) => hide_entry(&mut self.data, id),
            Message::ChangeNewEntryName(name) => self.new_entry_name = name,
            Message::CreateNew => {
                self.data.last_id += 1;
                let name = std::mem::take(&mut self.new_entry_name);
                let new_level = NpcConstructor::new(name.clone());
                super::item::save_by_id(&new_level, self.data.last_id);
                let new_entry = NpcInfo {
                    id: self.data.last_id,
                    name,
                    status: EntryStatus::Active,
                };
                self.data.list.push(new_entry);
                write_file(&self.data);
            }
            Message::Edit(id) => {
                return Some(super::Message::EditItem(id));
            }
        }
        None
    }
    pub fn view(&self) -> Element<Message> {
        let new_entry_row = row![
            text_input("New entry name", &self.new_entry_name)
                .on_input(Message::ChangeNewEntryName),
            button("Create new").on_press(Message::CreateNew),
        ];
        let heading_row = row![
            text(format!("Last item id: {}", self.data.last_id)),
            horizontal_space(),
            button("Refresh").on_press(Message::ReadFile),
        ];
        let mut details_column = column![heading_row, new_entry_row,]
            .align_x(Alignment::Start)
            .spacing(10);
        for item in self.data.list.iter().filter(|item| item.status.is_active()) {
            let item_row = row![
                text(format!("{}", item.id))
                    .width(portion(1))
                    .align_x(Alignment::Center),
                make_rule(1, Alignment::Start),
                text(&item.name).width(portion(5)),
                make_rule(1, Alignment::End),
                button(text("Edit").align_x(Alignment::Center))
                    .on_press(Message::Edit(item.id))
                    .width(portion(3)),
                button(text("Delete").align_x(Alignment::Center))
                    .on_press(Message::HideEntry(item.id))
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
