use iced::widget::{button, column, container, pick_list, row, text};
use iced::{Alignment, Element};

use shared::level::Level;

use super::get_item_file_path;

pub struct Page {
    id: u32,
    data: Level,
    selected: Option<u32>,
}

impl Page {
    pub fn load_by_id(id: u32) -> Self {
        Page {
            id,
            data: load_by_id(id),
            selected: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    WriteFile,
    SelectNpc(u32),
    AddNpc(u32),
    RemoveNpc(usize),
}

fn read_file() -> Option<Level> {
    let contents = std::fs::read("data/level.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(level: &Option<Level>) {
    let Some(level) = level else { return };
    let contents = serde_json::to_vec_pretty(level).expect("Should encode Level");
    std::fs::write("data/level.json", contents).expect("Should write Level to a file");
}

fn load_by_id(id: u32) -> Level {
    let file_path = get_item_file_path(id);
    let contents = std::fs::read(file_path).expect("Should read Level from a file");
    serde_json::from_slice(&contents).expect("Should decode Level")
}

pub fn save_by_id(level: &Level, id: u32) {
    let file_path = get_item_file_path(id);
    let contents = serde_json::to_vec_pretty(level).expect("Should encode Level");
    std::fs::write(file_path, contents).expect("Should write Level to a file");
}

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ReadFile => {
                self.data = load_by_id(self.id);
            }
            Message::WriteFile => save_by_id(&self.data, self.id),
            Message::SelectNpc(id) => {
                self.selected = Some(id);
            }
            Message::AddNpc(id) => {
                self.data.npc_list.push(id);
            }
            Message::RemoveNpc(index) => {
                if index >= self.data.npc_list.len() {
                    return;
                }
                self.data.npc_list.remove(index);
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

        let mut npc_list = column![].align_x(Alignment::Center).spacing(10);
        for (index, npc_id) in self.data.npc_list.iter().enumerate() {
            let npc_row = row![
                text(format!("Npc id: {}", npc_id)),
                button("delete").on_press(Message::RemoveNpc(index)),
            ]
            .spacing(10);
            npc_list = npc_list.push(npc_row);
        }
        let message_add = self.selected.map(Message::AddNpc);
        let add_npc_row = row![
            pick_list([1, 2, 3], self.selected, Message::SelectNpc),
            button("add").on_press_maybe(message_add),
        ]
        .spacing(10);
        let level_details_column = column![
            text("Add npc:"),
            add_npc_row,
            text("Level npc list:"),
            npc_list,
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let level_details = container(level_details_column).width(300);
        contents = contents.push(level_details);

        contents.into()
    }
}
