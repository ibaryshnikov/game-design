use std::time::Duration;

use axum::body::Bytes;
use axum::extract::ws::Message as WsMessage;
use tokio::sync::mpsc;
use tokio::time;

use game_core::server::ServerMessage;
use shared::types::Message;

use crate::broadcaster;
use crate::stage::Stage;
use crate::types::{GameLoopReceiver, LocalMessage, LoopMessage};

pub async fn game_loop(mut receiver: GameLoopReceiver) {
    let mut stage = Stage::new();

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
                            handle_message(id, *message, &broadcaster_sender).await;
                        }
                        LoopMessage::LocalMessage(id, message) => {
                            handle_local_message(&mut stage, id, *message, &broadcaster_sender).await;
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

async fn handle_message(
    id: u128,
    message: Message,
    broadcaster: &mpsc::Sender<broadcaster::Message>,
) {
    println!("Handle message in game loop for {id}");
    match message {
        Message::Join => {
            let m = ServerMessage::Test;
            let data = serde_json::to_vec(&m).expect("Should encode ServerMessage");
            let ws_message = WsMessage::Binary(Bytes::from(data));
            let new_message = broadcaster::Message::SendMessage(id, ws_message);
            if let Err(e) = broadcaster.send(new_message).await {
                tracing::error!("Failed to send message to broadcaster in game loop: {e}");
            }
        }
        Message::Move(kind, action) => {}
        Message::HeroDash => {}
        Message::HeroAttack => {}
        Message::Hero(hero) => {}
    }
}

async fn handle_local_message(
    stage: &mut Stage,
    id: u128,
    message: LocalMessage,
    broadcaster: &mpsc::Sender<broadcaster::Message>,
) {
    println!("Handle local message in game loop for {id}");
    match message {
        LocalMessage::Join => {
            println!("Got LocalMessage::Join");
            let scene = game_core::server::Scene {
                characters: stage.characters.clone(),
                npc: stage.npc.clone(),
            };
            let m = ServerMessage::Scene(scene);
            let data = rmp_serde::to_vec(&m).expect("Should encode ServerMessage");
            let ws_message = WsMessage::Binary(Bytes::from(data));
            let new_message = broadcaster::Message::SendMessage(id, ws_message);
            if let Err(e) = broadcaster.send(new_message).await {
                tracing::error!("Failed to send message to broadcaster in game loop: {e}");
            }
        }
    }
}
