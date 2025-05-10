use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element};

use shared::attack::{AttackConstructor, AttackKind, AttackOrder};

#[derive(Debug, Clone)]
pub enum Message {
    ReadFile,
    WriteFile,
    ChangeDelay(String),
    ChangeOrder(AttackOrder),
    ChangeKind(AttackKind),
}

fn read_file() -> Option<AttackConstructor> {
    let contents = std::fs::read("data/attack.json").ok()?;
    serde_json::from_slice(&contents).ok()
}

fn write_file(attack: &Option<AttackConstructor>) {
    let Some(attack) = attack else { return };
    let contents = serde_json::to_vec(attack).expect("Should encode AttackConstructor");
    std::fs::write("data/attack.json", contents).expect("Should write AttackConstructor to a file");
}

pub fn update(attack: &mut Option<AttackConstructor>, message: Message) {
    match message {
        Message::ReadFile => {
            let contents = read_file();
            if contents.is_some() {
                *attack = contents;
            } else {
                *attack = Some(AttackConstructor::default())
            }
        }
        Message::WriteFile => write_file(attack),
        Message::ChangeDelay(value) => {
            let parsed = value.parse::<u128>().ok();
            let Some(parsed) = parsed else {
                return;
            };
            if let Some(attack) = attack {
                attack.delay = parsed;
            }
        }
        Message::ChangeOrder(order) => {
            if let Some(attack) = attack {
                attack.order = order;
            }
        }
        Message::ChangeKind(kind) => {
            if let Some(attack) = attack {
                attack.kind = kind;
            }
        }
    }
}

pub fn view(attack: &Option<AttackConstructor>) -> Element<Message> {
    let mut contents = column![
        button("Read").on_press(Message::ReadFile),
        button("Write").on_press(Message::WriteFile),
    ]
    .align_x(Alignment::Center)
    .spacing(10);

    if let Some(attack) = attack {
        let attack_details_column = column![
            row![
                text("Delay"),
                text_input("Attack delay", &format!("{}", attack.delay))
                    .on_input(Message::ChangeDelay),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Order"),
                pick_list(
                    AttackOrder::options(),
                    Some(attack.order.clone()),
                    Message::ChangeOrder
                )
                .placeholder("Attack order"),
            ]
            .align_y(Alignment::Center)
            .spacing(10),
            row![
                text("Kind"),
                pick_list(
                    AttackKind::options(),
                    Some(attack.kind.clone()),
                    Message::ChangeKind
                )
                .placeholder("Attack kind")
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .align_x(Alignment::Start)
        .spacing(10);
        let attack_details = container(attack_details_column).width(300);
        contents = contents.push(attack_details);
    }

    contents.into()
}
