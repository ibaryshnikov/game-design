use std::fmt;

use bytes::Bytes;
use futures::sink::{Sink, SinkExt};
use futures::stream::StreamExt;
use futures_util::stream::SplitSink;
use iced_winit::winit::event_loop::EventLoopProxy;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite};
use tungstenite::{Error as WsError, Message as WsMessage};

use network::client::{self, KeyActionKind, Move};
use network::server;

use crate::UserEvent;
use crate::ui_app::Message;

type Writer = SplitSink<WebSocket, WsMessage>;

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
                KeyActionKind::Pressed => write!(f, "Move key pressed {movement:?}"),
                KeyActionKind::Released => write!(f, "Move key released {movement:?}"),
            },
            LocalMessage::HeroDash => write!(f, "HeroDash"),
            LocalMessage::HeroAttack => write!(f, "HeroAttack"),
        }
    }
}

const SERVER: &str = "ws://127.0.0.1:8080/ws";

pub async fn connect(proxy: EventLoopProxy<UserEvent>) {
    let (sender, mut receiver) = mpsc::channel(100);
    if let Err(e) = proxy.send_event(UserEvent::Message(Box::new(Message::WsChannel(sender)))) {
        println!("Error sending Message in ws: {e}");
    }

    let mut websocket;

    match tokio_tungstenite::connect_async(SERVER).await {
        Ok((ws, _)) => {
            websocket = ws;
            println!("WebSocket connected");
            if let Err(e) = proxy.send_event(UserEvent::Message(Box::new(Message::WsConnected))) {
                println!("Error sending Message in ws: {e}");
            }
        }
        Err(e) => {
            println!("Error connecting to WebSocket: {e}");
            if let Err(e) = proxy.send_event(UserEvent::Message(Box::new(Message::WsDisconnected)))
            {
                tracing::error!("Error sending Message in ws: {e}");
            }
            return;
        }
    }

    // let (mut write, mut read) = socket.split();
    // let mut maybe_ws_future = Option<Box::pin(websocket.next())>;
    // let mut maybe_receiver_future = Option<Box::pin(receiver.recv())>;

    loop {
        tokio::select! {
            maybe_message = websocket.next() => {
                match maybe_message {
                    Some(Ok(message)) => {
                        println!("Gow websocket message");
                        handle_ws_message(&proxy, message).await;
                    }
                    Some(Err(e)) => {
                        tracing::error!("Error reading WebSocket message: {e}");
                    }
                    None => {
                        println!("WebSocket closed by the server");
                        if let Err(e) = proxy.send_event(UserEvent::Message(Box::new(Message::WsDisconnected))) {
                            tracing::error!("Error sending Message in ws: {e}");
                        }
                        break;
                    }
                }
            }
            maybe_message = receiver.recv() => {
                if let Some(message) = maybe_message {
                    println!("Got local message in ws: {message:?}");
                    handle_local_message(&mut websocket, message).await;
                } else {
                    println!("Ws channel has been closed");
                    break;
                }
            }
        }
    }
}

// async fn ws_reader_loop<R>(reader: R, proxy: EventLoopProxy<UserEvent>)
// where
//     R: {
//
// }

type WebSocket = WebSocketStream<MaybeTlsStream<TcpStream>>;

async fn handle_local_message(websocket: &mut WebSocket, message: LocalMessage) {
    match message {
        LocalMessage::Move(kind, movement) => {
            println!("Local message Move kind {kind:?}: {movement:?}");
            let message = client::Message::Move(kind, movement);
            let bytes = Bytes::from(message.to_vec());
            let ws_message = WsMessage::Binary(bytes);
            if let Err(e) = websocket.send(ws_message).await {
                println!("Error sending WsMessage: {e:?}");
            }
        }
        LocalMessage::HeroDash => {
            println!("Local message HeroDash");
            let message = client::Message::HeroDash;
            let bytes = Bytes::from(message.to_vec());
            let ws_message = WsMessage::Binary(bytes);
            if let Err(e) = websocket.send(ws_message).await {
                println!("Error sending WsMessage: {e:?}");
            }
        }
        LocalMessage::HeroAttack => {
            println!("Local message HeroAttack");
            let message = client::Message::HeroAttack;
            let bytes = Bytes::from(message.to_vec());
            let ws_message = WsMessage::Binary(bytes);
            if let Err(e) = websocket.send(ws_message).await {
                println!("Error sending WsMessage: {e:?}");
            }
        }
        other => println!("Got some other message from WebSocket: {other:?}"),
    }
}

async fn handle_ws_message(proxy: &EventLoopProxy<UserEvent>, message: WsMessage) {
    match message {
        WsMessage::Text(text) => {
            println!("Got text message: {text}");
        }
        WsMessage::Binary(data) => {
            println!("Got binary data");
            let server_message = Box::new(server::Message::from_slice(&data));
            let event = UserEvent::Message(Box::new(Message::ServerMessage(server_message)));
            let _ = proxy.send_event(event);
        }
        other => {
            println!("Got other message: {other:?}");
        }
    }
}

async fn send_client_message<S>(write_half: &mut S, message: client::Message)
where
    S: Sink<WsMessage, Error = WsError> + Unpin,
{
    let data = message.to_vec();
    let ws_message = WsMessage::Binary(Bytes::from(data));
    if let Err(e) = write_half.send(ws_message).await {
        println!("Error sending WsMessage: {e:?}");
    }
}

async fn handle_socket_writer<S>(mut write_half: S, mut reader: mpsc::Receiver<LocalMessage>)
where
    S: Sink<WsMessage, Error = WsError> + Unpin,
{
    while let Some(message) = reader.recv().await {
        match message {
            LocalMessage::Move(kind, movement) => {
                println!("Local message Move kind {kind:?}: {movement:?}");
                let client_message = client::Message::Move(kind, movement);
                send_client_message(&mut write_half, client_message).await;
            }
            LocalMessage::HeroDash => {
                println!("Local message HeroDash");
                let client_message = client::Message::HeroDash;
                send_client_message(&mut write_half, client_message).await;
            }
            LocalMessage::HeroAttack => {
                println!("Local message HeroAttack");
                let client_message = client::Message::HeroAttack;
                send_client_message(&mut write_half, client_message).await;
            }
            other => println!("Got some other message in ws subscription: {other:?}"),
        }
    }
}
