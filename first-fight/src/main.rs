use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Engine, Renderer};
use iced_widget::canvas::{self, Cache, Canvas, Geometry};
use iced_widget::{button, column, container, row, text, Row};
use iced_winit::clipboard::Clipboard;
use iced_winit::core::{mouse, renderer, window, Event, Font, Pixels, Size, Theme};
use iced_winit::core::{Alignment, Element, Length, Rectangle};
use iced_winit::runtime::user_interface::{self, UserInterface};
use iced_winit::{conversion, winit};
use nalgebra::Point2;
use tokio::sync::mpsc;
use wgpu::{Device, Instance, Queue, TextureFormat};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Window, WindowId};

use shared::level::{Level, LevelInfo, LevelList};
use shared::npc::NpcConstructor;
use shared::types::{KeyActionKind, Move};

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

struct UiApp {
    proxy: EventLoopProxy<UserEvent>,
    cache: Cache,
    boss: Boss,
    hero: Hero,
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
    fn new(proxy: EventLoopProxy<UserEvent>) -> Self {
        let level_list = load_level_list();
        let boss_constructor = load_npc_by_id(1);
        let boss = Boss::from_constructor(Point2::new(512.0, 384.0), boss_constructor);
        let hero = Hero::new(Point2::new(250.0, 200.0));
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
        let npc_id = level.npc_list[0];
        let constructor = load_npc_by_id(npc_id);
        self.boss = Boss::from_constructor(Point2::new(512.0, 384.0), constructor);
    }
}

impl UiApp {
    fn update(&mut self, message: Message) {
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
                self.hero.reset();
                self.boss.reset();
                self.state = FightState::Action;
            }
            _ => (),
        }
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
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

fn key_event_to_message(event: &KeyEvent) -> Message {
    match event.physical_key {
        PhysicalKey::Code(code) => {
            let action = match code {
                KeyCode::KeyW => Move::Up,
                KeyCode::KeyS => Move::Down,
                KeyCode::KeyA => Move::Left,
                KeyCode::KeyD => Move::Right,
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    if let ElementState::Pressed = event.state {
                        return Message::HeroDash;
                    }
                    return Message::None;
                }
                KeyCode::Space => {
                    if let ElementState::Pressed = event.state {
                        return Message::HeroAttack;
                    }
                    return Message::None;
                }
                _ => return Message::None,
            };
            let kind = match event.state {
                ElementState::Pressed => KeyActionKind::Pressed,
                ElementState::Released => KeyActionKind::Released,
            };
            Message::Move(kind, action)
        }
        _ => Message::None,
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

#[derive(Debug)]
enum UserEvent {
    Tick,
    Message(Message),
}

struct App {
    proxy: EventLoopProxy<UserEvent>,
    app_data: Option<AppData>,
    ui_app: UiApp,
    events: Vec<Event>,
    cache: user_interface::Cache,
    cursor: mouse::Cursor,
    modifiers: ModifiersState,
    resized: bool,
}

struct AppData {
    window: Arc<Window>,
    device: Device,
    queue: Queue,
    surface: wgpu::Surface<'static>,
    format: TextureFormat,
    renderer: Renderer,
    clipboard: Clipboard,
    viewport: Viewport,
}

impl App {
    fn new(proxy: EventLoopProxy<UserEvent>) -> Self {
        let ui_app = UiApp::new(proxy.clone());
        Self {
            proxy,
            app_data: None,
            ui_app,
            events: vec![],
            cache: user_interface::Cache::new(),
            cursor: mouse::Cursor::Unavailable,
            modifiers: ModifiersState::default(),
            resized: false,
        }
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {
        // log::info!("New events cause {:?}", cause);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("Resumed");
        // if self.app_data.is_some() {
        //     log::info!("Already initialized, skipping");
        //     return;
        // }
        let proxy = self.proxy.clone();
        std::thread::spawn(move || {
            println!("Spawned a thread, creating a runtime");
            let rt = tokio::runtime::Runtime::new().expect("Should build a runtime");
            rt.block_on(ws::connect(proxy));
        });

        let instance = Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());

        let attrs = Window::default_attributes()
            .with_title("First fight")
            .with_inner_size(winit::dpi::LogicalSize::new(1000.0, 750.0));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());

        let physical_size = window.inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            window.scale_factor(),
        );
        let clipboard = Clipboard::connect(window.clone());

        let surface = instance
            .create_surface(window.clone())
            .expect("Create window surface");

        let (format, adapter, device, queue) = futures::executor::block_on(async {
            let adapter =
                wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
                    .await
                    .expect("Create adapter");

            let adapter_features = adapter.features();

            let capabilities = surface.get_capabilities(&adapter);

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: adapter_features & wgpu::Features::default(),
                        required_limits: wgpu::Limits::default(),
                        memory_hints: wgpu::MemoryHints::MemoryUsage,
                    },
                    None,
                )
                .await
                .expect("Request device");

            (
                capabilities
                    .formats
                    .iter()
                    .copied()
                    .find(wgpu::TextureFormat::is_srgb)
                    .or_else(|| capabilities.formats.first().copied())
                    .expect("Get preferred format"),
                adapter,
                device,
                queue,
            )
        });

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: physical_size.width,
                height: physical_size.height,
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            },
        );

        let engine = Engine::new(&adapter, device.clone(), queue.clone(), format, None);
        let renderer = Renderer::new(engine, Font::default(), Pixels::from(16));

        event_loop.set_control_flow(ControlFlow::Wait);

        self.cursor = mouse::Cursor::Unavailable;
        self.modifiers = ModifiersState::default();

        let app_data = AppData {
            window,
            device,
            queue,
            surface,
            format,
            renderer,
            clipboard,
            viewport,
        };
        self.app_data = Some(app_data);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::Tick => {
                self.ui_app.update(Message::Tick);
                self.request_redraw();
            }
            UserEvent::Message(message) => {
                self.ui_app.update(message);
                self.request_redraw();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // println!("Window event: {:?}", event);
        let Some(app_data) = self.app_data.as_mut() else {
            return;
        };

        let AppData {
            window,
            device,
            queue,
            surface,
            format,
            renderer,
            clipboard,
            ..
        } = app_data;

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if self.resized {
                    let size = window.inner_size();

                    app_data.viewport = Viewport::with_physical_size(
                        Size::new(size.width, size.height),
                        window.scale_factor(),
                    );

                    surface.configure(
                        device,
                        &wgpu::SurfaceConfiguration {
                            format: *format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::AutoVsync,
                            alpha_mode: wgpu::CompositeAlphaMode::Auto,
                            view_formats: vec![],
                            desired_maximum_frame_latency: 2,
                        },
                    );

                    self.resized = false;
                }

                match surface.get_current_texture() {
                    Ok(frame) => {
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });

                        queue.submit([encoder.finish()]);

                        let mut interface = UserInterface::build(
                            self.ui_app.view(),
                            app_data.viewport.logical_size(),
                            std::mem::take(&mut self.cache),
                            renderer,
                        );

                        let _ = interface.update(
                            &[Event::Window(window::Event::RedrawRequested(
                                std::time::Instant::now(),
                            ))],
                            self.cursor,
                            renderer,
                            clipboard,
                            &mut Vec::new(),
                        );

                        let mouse_interaction = interface.draw(
                            renderer,
                            &Theme::Ferra,
                            &renderer::Style::default(),
                            self.cursor,
                        );

                        self.cache = interface.into_cache();

                        renderer.present(None, frame.texture.format(), &view, &app_data.viewport);

                        frame.present();

                        window.set_cursor(conversion::mouse_interaction(mouse_interaction));
                    }
                    Err(error) => match error {
                        wgpu::SurfaceError::OutOfMemory => {
                            panic!(
                                "Swapchain error: {error}. \
                            Rendering cannot continue."
                            )
                        }
                        _ => {
                            window.request_redraw();
                        }
                    },
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = mouse::Cursor::Available(conversion::cursor_position(
                    position,
                    app_data.viewport.scale_factor(),
                ));
            }
            WindowEvent::Touch(touch) => {
                self.cursor = mouse::Cursor::Available(conversion::cursor_position(
                    touch.location,
                    app_data.viewport.scale_factor(),
                ));
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                ref event,
                is_synthetic: _,
            } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    match code {
                        KeyCode::ShiftLeft | KeyCode::ShiftRight => match event.state {
                            ElementState::Pressed => self.modifiers |= ModifiersState::SHIFT,
                            ElementState::Released => self.modifiers &= !ModifiersState::SHIFT,
                        },
                        KeyCode::ControlLeft | KeyCode::ControlRight => match event.state {
                            ElementState::Pressed => self.modifiers |= ModifiersState::CONTROL,
                            ElementState::Released => self.modifiers &= !ModifiersState::CONTROL,
                        },
                        _ => (),
                    }
                }
                let message = key_event_to_message(event);
                self.ui_app.update(message);
                window.request_redraw();
            }
            WindowEvent::Resized(_) => {
                self.resized = true;
            }
            _ => (),
        }

        if let Some(event) = conversion::window_event(event, window.scale_factor(), self.modifiers)
        {
            self.events.push(event);
        }

        self.update_controls();
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}
}

impl App {
    fn update_controls(&mut self) {
        if self.events.is_empty() {
            return;
        }

        let Some(app_data) = self.app_data.as_mut() else {
            return;
        };

        let AppData {
            window,
            renderer,
            viewport,
            clipboard,
            ..
        } = app_data;

        let mut interface = UserInterface::build(
            self.ui_app.view(),
            viewport.logical_size(),
            std::mem::take(&mut self.cache),
            renderer,
        );

        let mut messages = Vec::new();

        let _ = interface.update(
            &self.events,
            self.cursor,
            renderer,
            clipboard,
            &mut messages,
        );

        self.events.clear();
        self.cache = interface.into_cache();

        for message in messages {
            self.ui_app.update(message);
        }

        window.request_redraw();
    }
    fn request_redraw(&self) {
        let Some(app_data) = &self.app_data else {
            return;
        };
        app_data.window.request_redraw();
    }
}

fn main() {
    let event_loop = EventLoop::with_user_event()
        .build()
        .expect("Should build event loop");

    let proxy = event_loop.create_proxy();

    let new_proxy = proxy.clone();
    let _ = thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(15));
        let _ = new_proxy.send_event(UserEvent::Tick);
    });

    let mut app = App::new(proxy);
    event_loop.run_app(&mut app).expect("Should run event loop");
}
