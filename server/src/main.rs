use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{routing, Router};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use http::HeaderValue;
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;
// use tower_http::validate_request::ValidateRequestHeaderLayer;
use uuid::Uuid;

use shared::types::{KeyActionKind, Message as ServerMessage, Move};

mod broadcaster;
mod game_loop;
mod npc;
mod stage;
mod types;

use types::{GameLoopSender, LoopMessage};

struct AppState {
    game_loop_sender: GameLoopSender,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (sender, receiver) = mpsc::channel(1000);

    tokio::spawn(game_loop::game_loop(receiver));

    let state = AppState {
        game_loop_sender: sender,
    };

    let app = Router::new()
        .route("/login", routing::post(login_user))
        .route("/ws", routing::get(ws_handler))
        .route("/", routing::get(|| async { "hello from axum" }))
        .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        .with_state(Arc::new(state));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn login_user() -> String {
    "logged in".to_owned()
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let id = Uuid::new_v4().as_u128();
    println!("Got socket {id}");
    // share some data between sockets here
    // by passing it to the closure
    let sender = state.game_loop_sender.clone();
    ws.on_upgrade(move |socket| handle_socket(id, socket, sender))
}

async fn handle_socket(id: u128, socket: WebSocket, sender: GameLoopSender) {
    let (mut write, mut read) = socket.split();
    if write
        .send(Message::Ping(Bytes::from(vec![1, 2, 3])))
        .await
        .is_ok()
    {
        println!("Pinged!");
    } else {
        println!("Couldn't send ping to !");
        return;
    }
    let message = Box::new(broadcaster::Message::AddWriter(id, write));
    if let Err(e) = sender.send(LoopMessage::Broadcaster(message)).await {
        tracing::error!("Failed to send WebSocket writer to broadcaster: {e}");
        tracing::error!("Closing socket for {id}");
        return;
    }
    if let Some(maybe_message) = read.next().await {
        if let Ok(incoming_message) = maybe_message {
            println!("Got message {:?}", incoming_message);
        } else {
            println!("Client abruptly disconnected");
            return;
        }
    }

    let ws_message = Message::Text(Utf8Bytes::from("Hello from axum and ws".to_owned()));
    let message = Box::new(broadcaster::Message::SendMessage(id, ws_message));
    if let Err(e) = sender.send(LoopMessage::Broadcaster(message)).await {
        tracing::error!("Failed to send WebSocket message to broadcaster: {e}");
    }

    let message = ServerMessage::Move(KeyActionKind::Pressed, Move::Up);
    let data = message.to_vec();
    let ws_message = Message::Binary(Bytes::from(data));
    let message = Box::new(broadcaster::Message::SendMessage(id, ws_message));
    if let Err(e) = sender.send(LoopMessage::Broadcaster(message)).await {
        tracing::error!("Failed to send WebSocket message to broadcaster: {e}");
    }

    println!("Started reading loop");
    while let Some(maybe_message) = read.next().await {
        if let Ok(message) = maybe_message {
            match message {
                Message::Text(text) => println!("Got text message: {}", text),
                Message::Binary(data) => {
                    let message = ServerMessage::from_slice(&data);
                    println!("Got binary message: {:?}", message);
                    let result = sender
                        .send(LoopMessage::Server(id, Box::new(message)))
                        .await;
                    if let Err(e) = result {
                        println!("Error sending message: {:?}", e);
                        tracing::error!("Error sending message to game loop, receiver is dropped");
                        break;
                    }
                }
                other => println!("Got other ws message kind: {:?}", other),
            }
        }
    }

    let message = Box::new(broadcaster::Message::CloseConnection(id));
    if let Err(e) = sender.send(LoopMessage::Broadcaster(message)).await {
        tracing::error!("Failed to send WebSocket message to broadcaster: {e}");
    }
}
