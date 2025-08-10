use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element};

use shared::character::CharacterSettings;

use crate::utils::combine;
use crate::{DATA_PATH, EditorState};

const FILE_PREFIX: &str = "character";
const FOLDER_NAME: &str = "character/";
const FOLDER_PATH: &str = combine!(DATA_PATH, FOLDER_NAME);

fn get_item_file_path(id: u32) -> String {
    format!("{FOLDER_PATH}{FILE_PREFIX}_{id}.json")
}

// fn load_by_id(id: u32) -> CharacterSettings {
//     let file_path = get_item_file_path(id);
//     let contents = std::fs::read(file_path).expect("Should read CharacterSettings from a file");
//     serde_json::from_slice(&contents).expect("Should decode CharacterSettings")
// }

pub fn save_by_id(attack: &CharacterSettings, id: u32) {
    let file_path = get_item_file_path(id);
    let contents = serde_json::to_vec_pretty(attack).expect("Should encode CharacterSettings");
    std::fs::write(file_path, contents).expect("Should write CharacterSettings to a file");
}

fn read_file() -> Option<CharacterSettings> {
    let file_path = get_item_file_path(1); // only one character at the moment
    let contents = std::fs::read(file_path).ok()?;
    serde_json::from_slice(&contents).ok()
}

pub struct Page {
    item: CharacterSettings,
}

pub fn load_state() -> EditorState {
    EditorState::Character(Box::new(Page::load()))
}

impl Page {
    fn load() -> Self {
        Page {
            item: read_file().unwrap_or_default(),
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ReadFile => {
                self.item = read_file().unwrap_or_default();
            }
            Message::WriteFile => save_by_id(&self.item, 1),
            Message::ChangeDashDuration(value) => {
                let parsed = value.parse::<u128>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.item.dash_duration = parsed;
            }
            Message::ChangeDashDistance(value) => {
                let parsed = value.parse::<u128>().ok();
                let Some(parsed) = parsed else {
                    return;
                };
                self.item.dash_distance = parsed;
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
            row![
                text("Dash duration"),
                text_input("Dash duration", &format!("{}", self.item.dash_duration))
                    .on_input(Message::ChangeDashDuration),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Dash distance"),
                text_input("Dash distance", &format!("{}", self.item.dash_distance))
                    .on_input(Message::ChangeDashDistance),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let attack_details = container(attack_details_column).width(300);
        contents = contents.push(attack_details);

        contents.into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    WriteFile,
    ChangeDashDuration(String),
    ChangeDashDistance(String),
}
