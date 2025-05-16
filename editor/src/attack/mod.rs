use iced::Element;

pub mod item;
pub mod list;

pub struct Page {
    list: list::Page,
    item: Option<item::Page>,
    current_page: CurrentPage,
}

impl Page {
    pub fn load() -> Self {
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
            Message::EditAttack(id) => {
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
    EditAttack(u32),
    OpenList,
}

enum CurrentPage {
    Item,
    List,
}
