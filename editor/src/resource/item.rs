use iced::widget::{button, column, text, text_input, row};
use iced::{Alignment, Element};

use shared::resource::ResourceConstructor;

use super::get_item_file_path;
use crate::common::editor_row;

pub struct Page {
    id: u32,
    data: ResourceConstructor,
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
}

fn load_by_id(id: u32) -> ResourceConstructor {
    let file_path = get_item_file_path(id);
    let contents = std::fs::read(file_path).expect("Should read ResourceConstructor from a file");
    serde_json::from_slice(&contents).expect("Should decode ResourceConstructor")
}

pub fn save_by_id(resource: &ResourceConstructor, id: u32) {
    let file_path = get_item_file_path(id);
    let contents = serde_json::to_vec_pretty(resource).expect("Should encode ResourceConstructor");
    std::fs::write(file_path, contents).expect("Should write ResourceConstructor to a file");
}

pub(super) fn delete_file_by_id(id: u32) {
    let file_path = get_item_file_path(id);
    if let Err(e) = std::fs::remove_file(file_path) {
        println!("Error removing file for resource {id}: {e}");
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

        let resource_details_column = column![
            text(format!("Id {}", self.id)),
            editor_row(
                "Resource name",
                text_input("Resource name", &self.data.name).on_input(Message::ChangeName)
            ),
        ]
        .align_x(Alignment::Start)
        .spacing(10)
        .width(500);
        contents = contents.push(resource_details_column);

        contents.into()
    }
}
