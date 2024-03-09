use std::time::Duration;

use futures::channel::mpsc;
use iced::executor;
use iced::widget::canvas::{self, Cache, Canvas, Geometry};
use iced::widget::{button, column, row, text, Row};
use iced::{keyboard, mouse};
use iced::{
    Alignment, Application, Command, Element, Length, Rectangle, Renderer, Settings, Subscription,
    Theme,
};
use keyboard::key::{Key, Named};
use nalgebra::Point2;

use shared::types::Move;

mod attack;
mod boss;
mod hero;
mod ws;

use boss::Boss;
use hero::Hero;

#[derive(Debug, Clone)]
enum ServerAction {}

#[derive(Debug, Clone)]
enum Message {
    ConnectWs,
    WsChannel(mpsc::Sender<ws::LocalMessage>),
    WsConnected,
    WsDisconnected,
    WsMessage(String),
    ServerAction(ServerAction),
    Tick,
    Start,
    Retry,
    MoveStart(Move),
    MoveStop(Move),
    HeroDash,
    HeroAttack,
    None,
}

enum FightState {
    Pending,
    Action,
    Win,
    Loss,
}

struct App {
    cache: Cache,
    boss: Boss,
    hero: Hero,
    state: FightState,
    ws_sender: Option<mpsc::Sender<ws::LocalMessage>>,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let boss = Boss::new(Point2::new(512.0, 384.0));
        let hero = Hero::new(Point2::new(250.0, 200.0));
        let app = App {
            cache: Cache::new(),
            boss,
            hero,
            state: FightState::Pending,
            ws_sender: None,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "First fight".to_owned()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.cache.clear();

        match message {
            Message::WsChannel(sender) => {
                self.ws_sender = Some(sender);
            }
            Message::ConnectWs => {
                return Command::perform(ws::connect_ws_once(), |()| Message::None);
            }
            Message::WsConnected => {
                // do something, send some messages, idk
            }
            Message::WsDisconnected => {
                // do something here too
            }
            Message::ServerAction(action) => {
                // do nothing for now
            }
            Message::WsMessage(text) => {
                println!("Got ws message: {}", text);
            }
            Message::MoveStart(movement) => {
                self.hero.handle_move_keydown(movement.clone());
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.start_send(ws::LocalMessage::MoveStart(movement));
                }
            }
            Message::MoveStop(movement) => {
                self.hero.handle_move_keyup(movement.clone());
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.start_send(ws::LocalMessage::MoveStop(movement));
                }
            }
            Message::HeroDash => {
                self.hero.dash();
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.start_send(ws::LocalMessage::HeroDash);
                }
            }
            Message::HeroAttack => {
                self.hero.check_attack();
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.start_send(ws::LocalMessage::HeroAttack);
                }
            }
            Message::Tick => {
                self.hero.update(&mut self.boss);
                self.boss.update(&mut self.hero);
                if self.hero.hp <= 0 {
                    self.state = FightState::Loss;
                    self.hero.stop();
                    self.boss.stop();
                } else if self.boss.hp <= 0 {
                    self.state = FightState::Win;
                    self.hero.stop();
                    self.boss.stop();
                }
            }
            Message::Start => {
                self.state = FightState::Action;
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.start_send(ws::LocalMessage::ConnectWs);
                }
            }
            Message::Retry => {
                self.hero.reset();
                self.boss.reset();
                self.state = FightState::Action;
            }
            _ => (),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self.state {
            FightState::Pending => self.draw_pending().into(),
            FightState::Action => self.draw_action(),
            FightState::Win => self.draw_win().into(),
            FightState::Loss => self.draw_loss().into(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let timer = iced::time::every(Duration::from_millis(10)).map(|_| Message::Tick);
        let movements = iced::event::listen_with(|event, _status| match event {
            iced::Event::Keyboard(keyboard_event) => Some(keyboard_event),
            _ => None,
        })
        .map(key_event_to_message);
        let ws_events = ws::connect();
        Subscription::batch([timer, movements, ws_events])
    }
}

fn key_event_to_message(event: keyboard::Event) -> Message {
    match event {
        keyboard::Event::KeyPressed { key, .. } => {
            let action = match key.as_ref() {
                Key::Character("W" | "w") => Move::Up,
                Key::Character("S" | "s") => Move::Down,
                Key::Character("A" | "a") => Move::Left,
                Key::Character("D" | "d") => Move::Right,
                Key::Named(Named::Shift) => {
                    return Message::HeroDash;
                }
                Key::Named(Named::Space) => {
                    return Message::HeroAttack;
                }
                _ => return Message::None,
            };
            Message::MoveStart(action)
        }
        keyboard::Event::KeyReleased { key, .. } => {
            let action = match key.as_ref() {
                Key::Character("W" | "w") => Move::Up,
                Key::Character("S" | "s") => Move::Down,
                Key::Character("A" | "a") => Move::Left,
                Key::Character("D" | "d") => Move::Right,
                _ => return Message::None,
            };
            Message::MoveStop(action)
        }
        _ => Message::None,
    }
}

impl<Message> canvas::Program<Message> for App {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            self.boss.draw_body(frame);
            self.hero.draw_body(frame);
            self.boss.draw_attack(frame);
            self.hero.draw_attack(frame);
            self.boss.draw_health_bar(frame);
            self.hero.draw_health_bar(frame);
        });

        vec![geometry]
    }
}

impl App {
    fn draw_pending(&self) -> Row<Message> {
        let column = column![
            text("Welcome to the game!").size(30),
            button("Start").on_press(Message::Start),
            button("Connect ws").on_press(Message::ConnectWs),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill);
        row![column]
            .align_items(Alignment::Center)
            .height(Length::Fill)
    }
    fn draw_win(&self) -> Row<Message> {
        let column = column![
            text("You won, grab some loot!").size(30),
            button("Try again").on_press(Message::Retry),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill);
        row![column]
            .align_items(Alignment::Center)
            .height(Length::Fill)
    }
    fn draw_loss(&self) -> Row<Message> {
        let column = column![
            text("GAME OVER").size(40),
            button("Try again").on_press(Message::Retry),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill);
        row![column]
            .align_items(Alignment::Center)
            .height(Length::Fill)
    }
    fn draw_action(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        ..Default::default()
    })
}
