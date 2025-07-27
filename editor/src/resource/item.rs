use iced::widget::{Row, button, column, row, text, text_input};
use iced::{Alignment, Element};

use shared::resource::ResourceConstructor;

use super::get_item_file_path;

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

fn read_file() -> Option<ResourceConstructor> {
    let contents = std::fs::read("data/resource.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(resource: &Option<ResourceConstructor>) {
    let Some(resource) = resource else { return };
    let contents = serde_json::to_vec_pretty(resource).expect("Should encode ResourceConstructor");
    std::fs::write("data/resource.json", contents)
        .expect("Should write ResourceConstructor to a file");
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

impl Page {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ReadFile => {
                self.data = load_by_id(self.id);
            }
            Message::WriteFile => save_by_id(&self.data, self.id),
            Message::ChangeName(value) => {
                self.data.name = value;
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

        let resource_details_column = column![editor_row(
            "Resource name",
            text_input("Resource name", &self.data.name).on_input(Message::ChangeName),
        )]
        .align_x(Alignment::Start)
        .spacing(10)
        .width(500);
        contents = contents.push(resource_details_column);

        contents.into()
    }
}

fn editor_row<'a, T: Into<Element<'a, Message>>>(label: &'a str, element: T) -> Row<'a, Message> {
    row![text(label), element.into()]
        .align_y(Alignment::Center)
        .spacing(10)
}
