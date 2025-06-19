use tokio::sync::mpsc;

use shared::types::Message as ServerMessage;

use crate::broadcaster::Message as BroadcasterMessage;

pub type GameLoopSender = mpsc::Sender<LoopMessage>;
pub type GameLoopReceiver = mpsc::Receiver<LoopMessage>;

pub enum LocalMessage {
    Join,
}

pub enum LoopMessage {
    Broadcaster(Box<BroadcasterMessage>),
    Server(u128, Box<ServerMessage>),
    LocalMessage(u128, Box<LocalMessage>),
}
