use std::collections::HashMap;

use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures::sink::SinkExt;
use futures_util::stream::SplitSink;
use tokio::sync::mpsc::Receiver;

type Writer = SplitSink<WebSocket, WsMessage>;

pub enum Message {
    AddWriter(u128, Writer),
    RemoveWriter(u128),
    CloseConnection(u128),
    SendMessage(u128, WsMessage),
    SendMessageList(Vec<u128>, WsMessage),
    SendMessageToAll(WsMessage),
}

pub async fn start(mut receiver: Receiver<Message>) {
    let mut writers = HashMap::new();

    while let Some(message) = receiver.recv().await {
        handle_message(&mut writers, message).await;
    }
    panic!("Broadcaster channel was closed");
}

async fn handle_message(writers: &mut HashMap<u128, Writer>, message: Message) {
    use Message::*;
    match message {
        AddWriter(id, writer) => {
            println!("Adding writer for {id}");
            if let Some(_old) = writers.insert(id, writer) {
                tracing::warn!("Replaced websocket writer in broadcaster!");
            }
        }
        RemoveWriter(id) => {
            if writers.remove(&id).is_none() {
                tracing::warn!("Writer not found, can't remove in broadcaster!");
            }
        }
        CloseConnection(id) => close_writer(id, writers).await,
        SendMessage(id, message) => {
            send_message(id, writers, message).await;
        }
        SendMessageList(id_list, message) => {
            for id in id_list.into_iter() {
                send_message(id, writers, message.clone()).await;
            }
        }
        SendMessageToAll(message) => {
            for (id, writer) in writers.iter_mut() {
                // println!("Sending message in broadcaster to {id}");
                if let Err(e) = writer.send(message.clone()).await {
                    tracing::error!("Failed to send WebSocket message to id {id}: {e}");
                }
            }
        }
    }
}

async fn send_message(id: u128, writers: &mut HashMap<u128, Writer>, message: WsMessage) {
    let Some(writer) = writers.get_mut(&id) else {
        tracing::error!("Can't send message, writer for {id} not found in broadcaster!");
        return;
    };
    if let Err(e) = writer.send(message).await {
        tracing::error!("Failed to send WebSocket message to id {id}: {e}");
    }
}

async fn close_writer(id: u128, writers: &mut HashMap<u128, Writer>) {
    let Some(mut writer) = writers.remove(&id) else {
        tracing::warn!("Writer not found, can't close in broadcaster!");
        return;
    };
    let message = WsMessage::Close(None);
    if let Err(e) = writer.send(message).await {
        tracing::error!("Failed to send WebSocket message to id {id}: {e}");
    }
}
