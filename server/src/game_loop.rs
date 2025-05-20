use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time;

use shared::types::Message;

use crate::broadcaster;
use crate::stage::Stage;
use crate::types::{GameLoopReceiver, LoopMessage};

pub async fn game_loop(mut receiver: GameLoopReceiver) {
    let stage = Stage::new();

    let (broadcaster_sender, broadcaster_receiver) = mpsc::channel(100);

    tokio::spawn(broadcaster::start(broadcaster_receiver));

    loop {
        tokio::select! {
            maybe_message = receiver.recv() => {
                if let Some(message) = maybe_message {
                    match message {
                        LoopMessage::Broadcaster(message) => {
                            if let Err(e) = broadcaster_sender.send(*message).await {
                                tracing::error!("Failed to send message to broadcaster in game loop: {e}");
                            }
                        }
                        LoopMessage::Server(id, message) => {
                            handle_message(id, *message).await;
                        }
                    }
                } else {
                    panic!("Receiver is empty, game loop channel is closed");
                }
            }
            _ = time::sleep(Duration::from_millis(10)) => {

            }
        }
    }
}

async fn timer() {}

async fn handle_message(id: u128, message: Message) {
    println!("Handle message in game loop for {id}");
    match message {
        Message::Move(kind, action) => {}
        Message::HeroDash => {}
        Message::HeroAttack => {}
        Message::Hero(hero) => {}
    }
}
