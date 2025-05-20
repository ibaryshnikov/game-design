use iced::Element;

use crate::utils::combine;
use crate::{EditorState, DATA_PATH};

const FILE_PREFIX: &str = "attack";
const FOLDER_NAME: &str = "attack/";
const FOLDER_PATH: &str = combine!(DATA_PATH, FOLDER_NAME);

mod item;
pub mod list;

fn get_item_file_path(id: u32) -> String {
    format!("{FOLDER_PATH}/{FILE_PREFIX}_{id}.json")
}

pub struct Page {
    list: list::Page,
    item: Option<item::Page>,
    current_page: CurrentPage,
}

pub fn load_state() -> EditorState {
    EditorState::Attack(Box::new(Page::load()))
}

impl Page {
    fn load() -> Self {
        Page {
            list: list::Page::load(),
            item: None,
            current_page: CurrentPage::List,
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Item(message) => {
                if let Some(page) = &mut self.item {
                    page.update(message);
                }
            }
            Message::List(message) => {
                if let Some(new_message) = self.list.update(message) {
                    self.update(new_message);
                }
            }
            Message::EditItem(id) => {
                self.item = Some(item::Page::load_by_id(id));
                self.current_page = CurrentPage::Item;
            }
            Message::OpenList => {
                self.item = None;
                self.current_page = CurrentPage::List;
            }
        }
    }
    pub fn view(&self) -> Element<Message> {
        match self.current_page {
            CurrentPage::List => self.list.view().map(Message::List),
            CurrentPage::Item => {
                // should never be None here
                if let Some(item) = &self.item {
                    item.view().map(Message::Item)
                } else {
                    self.list.view().map(Message::List)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Item(item::Message),
    List(list::Message),
    EditItem(u32),
    OpenList,
}

enum CurrentPage {
    Item,
    List,
}
