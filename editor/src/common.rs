use iced::widget::{Row, row, text};
use iced::{Alignment, Element};

pub fn editor_row<'a, M, T: Into<Element<'a, M>>>(label: &'a str, element: T) -> Row<'a, M> {
    row![text(label), element.into()]
        .align_y(Alignment::Center)
        .spacing(10)
}
