use std::time::Duration;

use axum::body::Bytes;
use axum::extract::ws::Message as WsMessage;
use tokio::sync::mpsc;
use tokio::time;

use network::client;
use network::server;

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
                        LoopMessage::Client(id, message) => {
                            handle_client_message(&mut stage, id, *message, &broadcaster_sender).await;
                        }
                        LoopMessage::LocalMessage(id, message) => {
                            handle_local_message(&mut stage, id, *message, &broadcaster_sender).await;
                        }
                    }
                } else {
                    panic!("Receiver is empty, game loop channel is closed");
                }
            }
            _ = timer() => {
                stage.update();
            }
        }
    }
}

async fn timer() {
    tokio::time::sleep(Duration::from_millis(10)).await;
}

async fn handle_client_message(
    stage: &mut Stage,
    id: u128,
    message: client::Message,
    broadcaster: &mpsc::Sender<broadcaster::Message>,
) {
    println!("Handle message in game loop for {id}");
    stage.scene.handle_client_message(id, message);
    let scene = stage.scene.to_network();
    println!("network scene characters len {}", scene.characters.len());
    let server_message = server::Message::Update(server::Update::Scene(scene));
    let data = server_message.to_vec();
    let ws_message = WsMessage::Binary(Bytes::from(data));
    let new_message = broadcaster::Message::SendMessageToAll(ws_message);
    if let Err(e) = broadcaster.send(new_message).await {
        println!("Failed to send message to broadcaster in game loop handle_client_message: {e}");
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
            stage.scene.add_character(id);
            let scene = stage.scene.to_network();
            let server_message = server::Message::Update(server::Update::Scene(scene));
            let data = server_message.to_vec();
            let ws_message = WsMessage::Binary(Bytes::from(data));
            let new_message = broadcaster::Message::SendMessageToAll(ws_message);
            if let Err(e) = broadcaster.send(new_message).await {
                println!("Failed to send message to broadcaster in game loop handle_local_message: {e}");
            }
        }
    }
}
