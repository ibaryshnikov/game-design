use tokio::sync::mpsc;

use network::client;

use crate::broadcaster::Message as BroadcasterMessage;

pub type GameLoopSender = mpsc::Sender<LoopMessage>;
pub type GameLoopReceiver = mpsc::Receiver<LoopMessage>;

pub enum LocalMessage {
    Join,
}

pub enum LoopMessage {
    Broadcaster(Box<BroadcasterMessage>),
    Client(u128, Box<client::Message>),
    LocalMessage(u128, Box<LocalMessage>),
}
