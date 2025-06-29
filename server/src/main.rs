use std::sync::Arc;

use axum::body::Bytes;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::{Router, routing};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use http::HeaderValue;
use tokio::sync::mpsc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
// use tower_http::validate_request::ValidateRequestHeaderLayer;
use uuid::Uuid;

use network::client;
use network::server;

mod broadcaster;
mod config;
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

    let config = config::Config::read_from_file();

    let (sender, receiver) = mpsc::channel(1000);

    tokio::spawn(game_loop::game_loop(receiver));

    let state = AppState {
        game_loop_sender: sender,
    };

    let app = Router::new()
        .nest_service("/game", serve_web_client())
        .route("/login", routing::post(login_user))
        .route("/ws", routing::get(ws_handler))
        .route("/", routing::get(|| async { "hello from axum\n" }))
        .layer(CorsLayer::new().allow_origin("*".parse::<HeaderValue>().unwrap()))
        .with_state(Arc::new(state));
    let address = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("listening at {address}");
    axum::serve(listener, app).await.unwrap();
}

fn serve_web_client() -> ServeDir {
    ServeDir::new("../client-web/dist")
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

    let data = server::Message::SetId(id).to_vec();
    let ws_message = Message::Binary(Bytes::from(data));
    if let Err(e) = write.send(ws_message).await {
        tracing::error!("Failed to send SetId message to client: {e}");
    }

    let message = Box::new(broadcaster::Message::AddWriter(id, write));
    if let Err(e) = sender.send(LoopMessage::Broadcaster(message)).await {
        tracing::error!("Failed to send WebSocket writer to broadcaster: {e}");
        tracing::error!("Closing socket for {id}");
        return;
    }

    if let Err(e) = sender.send(LoopMessage::Join(id)).await {
        tracing::error!("Failed to send LoopMessage::LocalMessage: {e}");
    }

    println!("Started reading loop");
    while let Some(maybe_message) = read.next().await {
        if let Ok(message) = maybe_message {
            match message {
                Message::Text(text) => println!("Got text message: {text}"),
                Message::Binary(data) => {
                    let message = client::Message::from_slice(&data);
                    // println!("Got binary message: {message:?}");
                    let result = sender
                        .send(LoopMessage::Client(id, Box::new(message)))
                        .await;
                    if let Err(e) = result {
                        println!("Error sending message: {e:?}");
                        tracing::error!("Error sending message to game loop, receiver is dropped");
                        break;
                    }
                }
                other => println!("Got other ws message kind: {other:?}"),
            }
        }
    }

    if let Err(e) = sender.send(LoopMessage::Leave(id)).await {
        tracing::error!("Failed to send WebSocket message to broadcaster: {e}");
    }
}
