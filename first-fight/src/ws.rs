use std::fmt;

use bytes::Bytes;
use futures::sink::{Sink, SinkExt};
use futures::stream::StreamExt;
use iced_winit::winit::event_loop::EventLoopProxy;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Sender};
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};
use tungstenite::{Error as WsError, Message as WsMessage};

use shared::types::{KeyActionKind, Message as SharedMessage, Move};

use crate::{Message, UserEvent};

#[derive(Debug, Clone)]
pub enum LocalMessage {
    Connected,
    Disconnected,
    User(String),
    Move(KeyActionKind, Move),
    HeroDash,
    HeroAttack,
}

impl LocalMessage {
    pub fn new(message: &str) -> Option<Self> {
        if message.is_empty() {
            None
        } else {
            Some(Self::User(message.to_string()))
        }
    }

    pub fn connected() -> Self {
        LocalMessage::Connected
    }

    pub fn disconnected() -> Self {
        LocalMessage::Disconnected
    }
}

impl fmt::Display for LocalMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // LocalMessage::ConnectWs => write!(f, "Connect to ws command"),
            LocalMessage::Connected => write!(f, "Connected successfully!"),
            LocalMessage::Disconnected => {
                write!(f, "Connection lost... Retrying...")
            }
            LocalMessage::User(message) => write!(f, "{message}"),
            LocalMessage::Move(kind, movement) => match kind {
                KeyActionKind::Pressed => write!(f, "Move key pressed {:?}", movement),
                KeyActionKind::Released => write!(f, "Move key released {:?}", movement),
            },
            LocalMessage::HeroDash => write!(f, "HeroDash"),
            LocalMessage::HeroAttack => write!(f, "HeroAttack"),
        }
    }
}

const SERVER: &str = "ws://127.0.0.1:8080/ws";

pub async fn connect(proxy: EventLoopProxy<UserEvent>) {
    let (sender, mut receiver) = mpsc::channel(100);
    if let Err(e) = proxy.send_event(UserEvent::Message(Message::WsChannel(sender))) {
        println!("Error sending Message in ws: {}", e);
    }

    let mut websocket;

    match tokio_tungstenite::connect_async(SERVER).await {
        Ok((ws, _)) => {
            websocket = ws;
            println!("WebSocket connected");
            if let Err(e) = proxy.send_event(UserEvent::Message(Message::WsConnected)) {
                println!("Error sending Message in ws: {}", e);
            }
        }
        Err(e) => {
            println!("Error connecting to WebSocket: {}", e);
            if let Err(e) = proxy.send_event(UserEvent::Message(Message::WsDisconnected)) {
                tracing::error!("Error sending Message in ws: {}", e);
            }
            return;
        }
    }

    loop {
        tokio::select! {
            maybe_message = websocket.next() => {
                match maybe_message {
                    Some(Ok(message)) => {
                        println!("Gow websocket message: {:?}", message);
                        handle_ws_message(&proxy, message).await;
                    }
                    Some(Err(e)) => {
                        tracing::error!("Error reading WebSocket message: {}", e);
                    }
                    None => {
                        println!("WebSocket closed by the server");
                        if let Err(e) = proxy.send_event(UserEvent::Message(Message::WsDisconnected)) {
                            tracing::error!("Error sending Message in ws: {}", e);
                            break;
                        }
                    }
                }
            }
            maybe_message = receiver.recv() => {
                if let Some(message) = maybe_message {
                    println!("Got local message in ws: {:?}", message);
                    handle_local_message(&mut websocket, message).await;
                } else {
                    println!("Ws channel has been closed");
                    break;
                }
            }
        }
    }
}

type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

async fn handle_local_message(websocket: &mut WebSocket, message: LocalMessage) {
    match message {
        LocalMessage::Move(kind, movement) => {
            println!("Local message Move kind {:?}: {:?}", kind, movement);
            let message = SharedMessage::Move(kind, movement);
            let bytes = Bytes::from(message.to_vec());
            let ws_message = WsMessage::Binary(bytes);
            if let Err(e) = websocket.send(ws_message).await {
                println!("Error sending WsMessage: {:?}", e);
            }
        }
        LocalMessage::HeroDash => {
            println!("Local message HeroDash");
            let message = SharedMessage::HeroDash;
            let bytes = Bytes::from(message.to_vec());
            let ws_message = WsMessage::Binary(bytes);
            if let Err(e) = websocket.send(ws_message).await {
                println!("Error sending WsMessage: {:?}", e);
            }
        }
        LocalMessage::HeroAttack => {
            println!("Local message HeroAttack");
            let message = SharedMessage::HeroAttack;
            let bytes = Bytes::from(message.to_vec());
            let ws_message = WsMessage::Binary(bytes);
            if let Err(e) = websocket.send(ws_message).await {
                println!("Error sending WsMessage: {:?}", e);
            }
        }
        other => println!("Got some other message from WebSocket: {:?}", other),
    }
}

async fn handle_ws_message(_proxy: &EventLoopProxy<UserEvent>, message: WsMessage) {
    match message {
        WsMessage::Text(text) => {
            println!("Got text message: {}", text);
        }
        WsMessage::Binary(data) => {
            println!("Got binary data: {:?}", data);
        }
        other => {
            println!("Got other message: {:?}", other);
        }
    }
}

async fn handle_socket_writer<S>(mut write_half: S, mut reader: mpsc::Receiver<LocalMessage>)
where
    S: Sink<WsMessage, Error = WsError> + Unpin,
{
    while let Some(message) = reader.recv().await {
        match message {
            LocalMessage::Move(kind, movement) => {
                println!("Local message Move kind {:?}: {:?}", kind, movement);
                let message = SharedMessage::Move(kind, movement);
                let bytes = Bytes::from(message.to_vec());
                let ws_message = WsMessage::Binary(bytes);
                if let Err(e) = write_half.send(ws_message).await {
                    println!("Error sending WsMessage: {:?}", e);
                }
            }
            LocalMessage::HeroDash => {
                println!("Local message HeroDash");
                let message = SharedMessage::HeroDash;
                let bytes = Bytes::from(message.to_vec());
                let ws_message = WsMessage::Binary(bytes);
                if let Err(e) = write_half.send(ws_message).await {
                    println!("Error sending WsMessage: {:?}", e);
                }
            }
            LocalMessage::HeroAttack => {
                println!("Local message HeroAttack");
                let message = SharedMessage::HeroAttack;
                let bytes = Bytes::from(message.to_vec());
                let ws_message = WsMessage::Binary(bytes);
                if let Err(e) = write_half.send(ws_message).await {
                    println!("Error sending WsMessage: {:?}", e);
                }
            }
            other => println!("Got some other message in ws subscription: {:?}", other),
        }
    }
}
