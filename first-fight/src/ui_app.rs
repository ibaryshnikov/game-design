use std::collections::HashMap;
use std::time::Instant;

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
use game_core::scene::Scene;
use network::client::{KeyActionKind, Move};
use network::server;
use shared::level::{Level, LevelInfo, LevelList};
use shared::npc::NpcConstructor;

use crate::hero::HeroView;
use crate::scene::SceneView;
use crate::{ws, UserEvent};

#[derive(Debug, Clone)]
pub enum ServerAction {}

#[derive(Debug, Clone)]
pub enum Message {
    WsChannel(mpsc::Sender<ws::LocalMessage>),
    WsConnected,
    WsDisconnected,
    WsMessage(String),
    UpdateScene(server::Scene),
    ServerAction(ServerAction),
    ServerMessage(Box<server::Message>),
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
    last_update: Instant,
    proxy: EventLoopProxy<UserEvent>,
    cache: Cache,
    hero: Hero,
    scene: Scene,
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
        // let boss = Boss::from_constructor(Point2::new(512.0, 384.0), boss_constructor);
        let hero = Hero::new(Point2::new(250.0, 200.0));
        let scene = Scene::empty();
        // let scene = Scene::new(hero.clone(), boss);
        UiApp {
            last_update: Instant::now(),
            proxy,
            cache: Cache::new(),
            hero,
            scene,
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
        self.scene.npc = level
            .npc_list
            .iter()
            .map(|npc| {
                let constructor = load_npc_by_id(npc.id);
                Boss::from_constructor(Point2::new(512.0, 384.0), constructor)
            })
            .collect();
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
            Message::UpdateScene(scene) => {
                self.scene.update_from_network(scene);
            }
            Message::ServerAction(_action) => {
                // do nothing for now
            }
            Message::ServerMessage(m) => {
                self.scene.handle_server_message(*m);
            }
            Message::WsMessage(text) => {
                println!("Got ws message: {}", text);
            }
            Message::Move(kind, movement) => {
                // println!("Moving: {:?} {:?}", kind, movement);
                self.hero.handle_move_action(kind.clone(), movement.clone());
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.try_send(ws::LocalMessage::Move(kind, movement));
                }
            }
            Message::HeroDash => {
                self.hero.dash();
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.try_send(ws::LocalMessage::HeroDash);
                }
            }
            Message::HeroAttack => {
                self.hero.check_attack();
                if let Some(sender) = &mut self.ws_sender {
                    let _ = sender.try_send(ws::LocalMessage::HeroAttack);
                }
            }
            Message::Tick => {
                let now = Instant::now();
                let dt = now.saturating_duration_since(self.last_update).as_millis();
                self.last_update = now;
                self.hero.update_visuals(dt);
                self.scene.update(dt);
                // if self.scene.characters.values().all(|hero| hero.defeated()) {
                //     self.state = FightState::Loss;
                //     self.scene.stop();
                // } else if self.scene.npc.iter().all(|boss| boss.defeated()) {
                //     self.state = FightState::Win;
                //     self.scene.stop();
                // }
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
                self.scene.reset();
                self.state = FightState::Action;
            }
            Message::None => (), // do nothing
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
            HeroView::new(&self.hero).draw(frame);
            SceneView::new(&self.scene).draw(frame);
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
