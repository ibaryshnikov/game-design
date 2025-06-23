use std::time::Duration;

use axum::body::Bytes;
use axum::extract::ws::Message as WsMessage;
use tokio::sync::mpsc;
use tokio::time;

use network::client;
use network::server;

use crate::broadcaster;
use crate::stage::Stage;
use crate::types::{GameLoopReceiver, LoopMessage};

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
                        LoopMessage::Join(id) => {
                            handle_character_join(&mut stage, id, &broadcaster_sender).await;
                        }
                        LoopMessage::Leave(id) => {
                            let message = broadcaster::Message::CloseConnection(id);
                            if let Err(e) = broadcaster_sender.send(message).await {
                                tracing::error!("Failed to send message to broadcaster in game loop: {e}");
                            }
                            handle_character_leave(&mut stage, id, &broadcaster_sender).await;
                        }
                    }
                } else {
                    panic!("Receiver is empty, game loop channel is closed");
                }
            }
            _ = timer() => {
                if stage.update() {
                    send_scene_to_clients(&stage, &broadcaster_sender).await;
                }
            }
        }
    }
}

async fn timer() {
    time::sleep(Duration::from_millis(10)).await;
}

async fn send_scene_to_clients(stage: &Stage, broadcaster: &mpsc::Sender<broadcaster::Message>) {
    let scene = stage.scene.to_network();
    let server_message = server::Message::Update(server::Update::Scene(scene));
    let data = server_message.to_vec();
    let ws_message = WsMessage::Binary(Bytes::from(data));
    let new_message = broadcaster::Message::SendMessageToAll(ws_message);
    if let Err(e) = broadcaster.send(new_message).await {
        println!("Failed to send Scene to broadcaster in game loop: {e}");
    }
}

async fn handle_client_message(
    stage: &mut Stage,
    id: u128,
    message: client::Message,
    broadcaster: &mpsc::Sender<broadcaster::Message>,
) {
    println!("Handle message in game loop for {id}");
    stage.scene.handle_client_message(id, message);
    send_scene_to_clients(stage, broadcaster).await;
}

async fn handle_character_join(
    stage: &mut Stage,
    id: u128,
    broadcaster: &mpsc::Sender<broadcaster::Message>,
) {
    println!("Got LoopMessage::Join");
    stage.scene.add_character(id);
    let scene = stage.scene.to_network();
    let server_message = server::Message::Update(server::Update::Scene(scene));
    let data = server_message.to_vec();
    let ws_message = WsMessage::Binary(Bytes::from(data));
    let new_message = broadcaster::Message::SendMessageToAll(ws_message);
    if let Err(e) = broadcaster.send(new_message).await {
        println!("Failed to send message to broadcaster in game loop handle_character_join: {e}");
    }
}

async fn handle_character_leave(
    stage: &mut Stage,
    id: u128,
    broadcaster: &mpsc::Sender<broadcaster::Message>,
) {
    println!("Got LoopMessage::Leave");
    stage.scene.remove_character(id);
    let scene = stage.scene.to_network();
    let server_message = server::Message::Update(server::Update::Scene(scene));
    let data = server_message.to_vec();
    let ws_message = WsMessage::Binary(Bytes::from(data));
    let new_message = broadcaster::Message::SendMessageToAll(ws_message);
    if let Err(e) = broadcaster.send(new_message).await {
        println!("Failed to send message to broadcaster in game loop handle_character_leave: {e}");
    }
}
