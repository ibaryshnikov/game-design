use std::collections::HashMap;

use iced_wgpu::Renderer;
use iced_widget::canvas::{self, Cache, Canvas, Geometry};
use iced_widget::{button, column, container, row, text, Row};
use iced_winit::core::{mouse, Alignment, Element, Length, Rectangle, Theme};
use iced_winit::winit;
use nalgebra::Point2;
use tokio::sync::mpsc;
use winit::event_loop::EventLoopProxy;

use game_core::boss::Boss;
use game_core::hero::Hero;
use shared::level::{Level, LevelInfo, LevelList};
use shared::npc::NpcConstructor;
use shared::types::{KeyActionKind, Move};

use crate::boss::BossView;
use crate::hero::HeroView;
use crate::{ws, UserEvent};

#[derive(Debug, Clone)]
pub enum ServerAction {}

#[derive(Debug, Clone)]
pub enum Message {
    WsChannel(mpsc::Sender<ws::LocalMessage>),
    WsConnected,
    WsDisconnected,
    WsMessage(String),
    ServerAction(ServerAction),
    Tick,
    SwitchToLevelSelect,
    Start(u32),
    SelectLevel(u32),
    Retry,
    Move(KeyActionKind, Move),
    HeroDash,
    HeroAttack,
    None,
}

enum FightState {
    Pending,
    LevelSelect,
    Action,
    Win,
    Loss,
}

pub struct UiApp {
    proxy: EventLoopProxy<UserEvent>,
    cache: Cache,
    boss: BossView,
    hero: HeroView,
    characters: HashMap<u128, Hero>,
    npc_list: HashMap<u128, Boss>,
    state: FightState,
    ws_sender: Option<mpsc::Sender<ws::LocalMessage>>,
    level_list: LevelList,
    selected_level: Option<LevelInfo>,
}

fn load_npc_by_id(id: u32) -> NpcConstructor {
    let file_path = format!("../data/npc/npc_{id}.json");
    let contents = std::fs::read(file_path).expect("Should read NpcConstructor from a file");
    serde_json::from_slice(&contents).expect("Should decode NpcConstructor")
}

fn load_level_list() -> LevelList {
    let file_path = "../data/level/list.json";
    let contents = std::fs::read(file_path).expect("Should read LevelList from a file");
    serde_json::from_slice(&contents).expect("Should decode LevelList")
}

fn load_level_by_id(id: u32) -> Level {
    let file_path = format!("../data/level/level_{id}.json");
    let contents = std::fs::read(file_path).expect("Should read Level from a file");
    serde_json::from_slice(&contents).expect("Should decode Level")
}

impl UiApp {
    pub fn new(proxy: EventLoopProxy<UserEvent>) -> Self {
        let level_list = load_level_list();
        let boss_constructor = load_npc_by_id(1);
        let boss_info = Boss::from_constructor(Point2::new(512.0, 384.0), boss_constructor);
        let boss = BossView { boss_info };
        let hero_info = Hero::new(Point2::new(250.0, 200.0));
        let hero = HeroView { hero_info };
        UiApp {
            proxy,
            cache: Cache::new(),
            boss,
            hero,
            characters: HashMap::new(),
            npc_list: HashMap::new(),
            state: FightState::Pending,
            ws_sender: None,
            level_list,
            selected_level: None,
        }
    }
    fn load_level(&mut self, id: u32) {
        let level = load_level_by_id(id);
        let npc = &level.npc_list[0];
        let constructor = load_npc_by_id(npc.id);
        let boss_info = Boss::from_constructor(Point2::new(512.0, 384.0), constructor);
        self.boss = BossView { boss_info };
    }
}

impl UiApp {
    pub fn update(&mut self, message: Message) {
        self.cache.clear();

        match message {
            Message::WsChannel(sender) => {
                self.ws_sender = Some(sender);
            }
            Message::WsConnected => {
                // do something, send some messages, idk
            }
            Message::WsDisconnected => {
                // do something here too
            }
            Message::ServerAction(_action) => {
                // do nothing for now
            }
            Message::WsMessage(text) => {
                println!("Got ws message: {}", text);
            }
            Message::Move(kind, movement) => {
                // println!("Moving: {:?} {:?}", kind, movement);
                self.hero
                    .hero_info
                    .handle_move_action(kind.clone(), movement.clone());
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.try_send(ws::LocalMessage::Move(kind, movement));
                }
            }
            Message::HeroDash => {
                self.hero.hero_info.dash();
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.try_send(ws::LocalMessage::HeroDash);
                }
            }
            Message::HeroAttack => {
                self.hero.hero_info.check_attack();
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.try_send(ws::LocalMessage::HeroAttack);
                }
            }
            Message::Tick => {
                self.hero.hero_info.update(&mut self.boss.boss_info);
                self.boss.boss_info.update(&mut self.hero.hero_info);
                if self.hero.hero_info.defeated() {
                    self.state = FightState::Loss;
                    self.hero.hero_info.stop();
                    self.boss.boss_info.stop();
                } else if self.boss.boss_info.defeated() {
                    self.state = FightState::Win;
                    self.hero.hero_info.stop();
                    self.boss.boss_info.stop();
                }
            }
            Message::SwitchToLevelSelect => {
                self.state = FightState::LevelSelect;
            }
            Message::SelectLevel(id) => {
                self.selected_level = self
                    .level_list
                    .list
                    .iter()
                    .find(|item| item.id == id)
                    .cloned();
            }
            Message::Start(id) => {
                self.load_level(id);
                self.state = FightState::Action;
            }
            Message::Retry => {
                self.hero.hero_info.reset();
                self.boss.boss_info.reset();
                self.state = FightState::Action;
            }
            _ => (),
        }
    }

    pub fn view(&self) -> Element<Message, Theme, Renderer> {
        let el = match self.state {
            FightState::Pending => self.draw_pending().into(),
            FightState::LevelSelect => self.draw_level_selection().into(),
            FightState::Action => self.draw_action(),
            FightState::Win => self.draw_win().into(),
            FightState::Loss => self.draw_loss().into(),
        };
        container(el)
            .style(|theme| theme.palette().background.into())
            .into()
    }
}

impl<Message> canvas::Program<Message> for UiApp {
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

impl UiApp {
    fn draw_pending(&self) -> Row<Message> {
        let column = column![
            text("Welcome to the game!").size(30),
            button("Start").on_press(Message::SwitchToLevelSelect),
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill);
        row![column].align_y(Alignment::Center).height(Length::Fill)
    }
    fn draw_level_selection(&self) -> Row<Message> {
        let mut level_list = column![].align_x(Alignment::Center).height(Length::Fill);
        for item in self.level_list.list.iter() {
            let level = button(text(&item.name)).on_press(Message::SelectLevel(item.id));
            level_list = level_list.push(level);
        }
        let mut level_preview = column![].align_x(Alignment::Center).height(Length::Fill);
        if let Some(selected_level) = &self.selected_level {
            let preview = column![
                text(format!("Selected level: {}", selected_level.name)),
                button("Play").on_press(Message::Start(selected_level.id)),
            ];
            level_preview = level_preview.push(preview);
        }
        row![level_list, level_preview]
            .align_y(Alignment::Center)
            .height(Length::Fill)
    }
    fn draw_win(&self) -> Row<Message> {
        let column = column![
            text("You won, grab some loot!").size(30),
            button("Try again").on_press(Message::Retry),
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill);
        row![column].align_y(Alignment::Center).height(Length::Fill)
    }
    fn draw_loss(&self) -> Row<Message> {
        let column = column![
            text("GAME OVER").size(40),
            button("Try again").on_press(Message::Retry),
        ]
        .align_x(Alignment::Center)
        .width(Length::Fill);
        row![column].align_y(Alignment::Center).height(Length::Fill)
    }
    fn draw_action(&self) -> Element<Message, Theme, Renderer> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
