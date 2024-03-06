use futures::channel::mpsc;
use futures::sink::{Sink, SinkExt};
use futures::stream::{SplitStream, StreamExt};
use iced::subscription::{self, Subscription};
use std::fmt;
use tokio::io::AsyncRead;
use tokio_tungstenite::tungstenite::{Error as WsError, Message as WsMessage};
use tokio_tungstenite::WebSocketStream;

use shared::types::Message as SharedMessage;

use crate::{Message, Move};

#[derive(Debug, Clone)]
pub enum LocalMessage {
    ConnectWs,
    Connected,
    Disconnected,
    User(String),
    MoveStart(Move),
    MoveStop(Move),
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
            LocalMessage::ConnectWs => write!(f, "Connect to ws command"),
            LocalMessage::Connected => write!(f, "Connected successfully!"),
            LocalMessage::Disconnected => {
                write!(f, "Connection lost... Retrying...")
            }
            LocalMessage::User(message) => write!(f, "{message}"),
            LocalMessage::MoveStart(movement) => write!(f, "MoveStart {:?}", movement),
            LocalMessage::MoveStop(movement) => write!(f, "MoveStop {:?}", movement),
            LocalMessage::HeroDash => write!(f, "HeroDash"),
            LocalMessage::HeroAttack => write!(f, "HeroAttack"),
        }
    }
}

const SERVER: &str = "ws://127.0.0.1:8080/ws";

pub fn connect() -> Subscription<Message> {
    struct Connect;

    subscription::channel(
        std::any::TypeId::of::<Connect>(),
        100,
        |mut output| async move {
            let mut state = State::Disconnected;

            println!("Initiating ws subscription");
            let (sender, mut receiver) = mpsc::channel(100);
            let _ = output.send(Message::WsChannel(sender)).await;

            let maybe_message = receiver.next().await;
            if let Some(message) = maybe_message {
                match message {
                    LocalMessage::ConnectWs => {
                        println!("Connecting to WebSocket");
                        match tokio_tungstenite::connect_async(SERVER).await {
                            Ok((websocket, _)) => {
                                let _ = output.send(Message::WsConnected).await;
                                let (write, read) = websocket.split();

                                tokio::spawn(handle_socket_writer(write, receiver));

                                state = State::Connected(read);
                            }
                            Err(_) => {
                                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                                let _ = output.send(Message::WsDisconnected).await;
                            }
                        }
                    }
                    other => {
                        panic!("Unexpected local message: {:?}", other);
                    }
                }
            }

            loop {
                match &mut state {
                    State::Disconnected => {
                        let _ = output.send(Message::WsDisconnected).await;
                        state = State::Stale;
                    }
                    State::Connected(read_half) => {
                        while let Some(message) = read_half.next().await {
                            println!("Got ws message: {:?}", message);
                        }
                        let _ = output.send(Message::WsDisconnected).await;
                    }
                    State::Stale => {
                        println!("ws connection became stale");
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    }
                }
            }
        },
    )
}

async fn handle_socket_writer<S>(mut write_half: S, mut reader: mpsc::Receiver<LocalMessage>)
where
    S: Sink<WsMessage, Error = WsError> + Unpin,
{
    while let Some(message) = reader.next().await {
        match message {
            LocalMessage::MoveStart(movement) => {
                println!("Local message MoveStart: {:?}", movement);
                let message = SharedMessage::MoveKeyUp(movement);
                let data = message.to_vec();
                let ws_message = WsMessage::Binary(data);
                if let Err(e) = write_half.send(ws_message).await {
                    println!("Error sending WsMessage: {:?}", e);
                }
            }
            LocalMessage::MoveStop(movement) => {
                println!("Local message MoveStop: {:?}", movement);
                let message = SharedMessage::MoveKeyDown(movement);
                let data = message.to_vec();
                let ws_message = WsMessage::Binary(data);
                if let Err(e) = write_half.send(ws_message).await {
                    println!("Error sending WsMessage: {:?}", e);
                }
            }
            LocalMessage::HeroDash => {
                println!("Local message HeroDash");
                let message = SharedMessage::HeroDash;
                let data = message.to_vec();
                let ws_message = WsMessage::Binary(data);
                if let Err(e) = write_half.send(ws_message).await {
                    println!("Error sending WsMessage: {:?}", e);
                }
            }
            LocalMessage::HeroAttack => {
                println!("Local message HeroAttack");
                let message = SharedMessage::HeroAttack;
                let data = message.to_vec();
                let ws_message = WsMessage::Binary(data);
                if let Err(e) = write_half.send(ws_message).await {
                    println!("Error sending WsMessage: {:?}", e);
                }
            }
            other => println!("Got some other message in ws subscription: {:?}", other),
        }
    }
}

pub async fn connect_ws_once() {
    let res = connect_ws().await;
    if let Err(e) = res {
        println!("Error connecting to ws server: {:?}", e);
    }
}

async fn connect_ws() -> anyhow::Result<()> {
    let (ws_stream, _response) = tokio_tungstenite::connect_async(SERVER).await?;
    let (_write, mut read) = ws_stream.split();
    while let Some(maybe_message) = read.next().await {
        match maybe_message {
            Ok(message) => handle_ws_message(message),
            Err(e) => println!("Error reading from WebSocket: {}", e),
        }
    }
    Ok(())
}

fn handle_ws_message(message: WsMessage) {
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

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum State<S: AsyncRead + Unpin> {
    Disconnected,
    Connected(SplitStream<WebSocketStream<S>>),
    Stale,
}
