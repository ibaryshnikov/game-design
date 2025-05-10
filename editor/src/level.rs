use iced::widget::{button, column, container, pick_list, row, text};
use iced::{Alignment, Element};

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    WriteFile,
    SelectNpc(i32),
    AddNpc(i32),
    RemoveNpc(usize),
}

#[derive(Default, Debug)]
pub struct Level {
    npc_list: Vec<i32>,
    selected: Option<i32>,
}

fn read_file() -> Option<Level> {
    let contents = std::fs::read("data/level.json").ok()?;
    let npc_list = serde_json::from_slice(&contents).ok()?;
    let level = Level {
        npc_list,
        selected: None,
    };
    Some(level)
}

fn write_file(level: &Option<Level>) {
    let Some(level) = level else { return };
    let contents = serde_json::to_vec(&level.npc_list).expect("Should encode Level");
    std::fs::write("data/level.json", contents).expect("Should write Level to a file");
}

pub fn update(level: &mut Option<Level>, message: Message) {
    match message {
        Message::ReadFile => {
            let contents = read_file();
            if contents.is_some() {
                *level = contents;
            } else {
                *level = Some(Level::default())
            }
        }
        Message::WriteFile => write_file(level),
        Message::SelectNpc(id) => {
            if let Some(level) = level {
                level.selected = Some(id);
            }
        }
        Message::AddNpc(id) => {
            if let Some(level) = level {
                level.npc_list.push(id);
            }
        }
        Message::RemoveNpc(index) => {
            if let Some(level) = level {
                if index >= level.npc_list.len() {
                    return;
                }
                level.npc_list.remove(index);
            }
        }
    }
}

pub fn view(level: &Option<Level>) -> Element<Message> {
    let mut contents = column![
        button("Read").on_press(Message::ReadFile),
        button("Write").on_press(Message::WriteFile),
    ]
    .align_x(Alignment::Center)
    .spacing(10);

    if let Some(level) = level {
        let mut npc_list = column![].align_x(Alignment::Center).spacing(10);
        for (index, npc_id) in level.npc_list.iter().enumerate() {
            let npc_row = row![
                text(format!("Npc id: {}", npc_id)),
                button("delete").on_press(Message::RemoveNpc(index)),
            ]
            .spacing(10);
            npc_list = npc_list.push(npc_row);
        }
        let message_add = level.selected.map(Message::AddNpc);
        let add_npc_row = row![
            pick_list([1, 2, 3], level.selected, Message::SelectNpc,),
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
    }

    contents.into()
}
