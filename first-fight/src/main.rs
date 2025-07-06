use std::sync::Arc;
use std::thread;
use std::time::Duration;

use iced_wgpu::graphics::Viewport;
use iced_wgpu::{Engine, Renderer, wgpu};
use iced_winit::clipboard::Clipboard;
use iced_winit::core::{Event, Font, Pixels, Size, Theme, mouse, renderer, window};
use iced_winit::runtime::user_interface::{self, UserInterface};
use iced_winit::{conversion, winit};
use wgpu::{Device, Instance, Queue, TextureFormat};
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::keyboard::{KeyCode, ModifiersState, PhysicalKey};
use winit::window::{Window, WindowId};

use network::client;

mod attack;
mod boss;
mod hero;
mod scene;
mod ui_app;
mod ws;

use ui_app::UiApp;

fn key_event_to_message(event: &KeyEvent) -> ui_app::Message {
    match event.physical_key {
        PhysicalKey::Code(code) => {
            let action = match code {
                KeyCode::KeyW => client::Move::Up,
                KeyCode::KeyS => client::Move::Down,
                KeyCode::KeyA => client::Move::Left,
                KeyCode::KeyD => client::Move::Right,
                KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                    if let ElementState::Pressed = event.state {
                        return ui_app::Message::HeroDash;
                    }
                    return ui_app::Message::None;
                }
                KeyCode::Space => {
                    if let ElementState::Pressed = event.state {
                        return ui_app::Message::HeroAttack;
                    }
                    return ui_app::Message::None;
                }
                _ => return ui_app::Message::None,
            };
            let kind = match event.state {
                ElementState::Pressed => client::KeyActionKind::Pressed,
                ElementState::Released => client::KeyActionKind::Released,
            };
            ui_app::Message::Move(kind, action)
        }
        _ => ui_app::Message::None,
    }
}

#[derive(Debug)]
enum UserEvent {
    Tick,
    Message(Box<ui_app::Message>),
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
                self.ui_app.update(ui_app::Message::Tick);
                self.request_redraw();
            }
            UserEvent::Message(message) => {
                self.ui_app.update(*message);
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

                        let (state, _) = interface.update(
                            &[Event::Window(window::Event::RedrawRequested(
                                std::time::Instant::now(),
                            ))],
                            self.cursor,
                            renderer,
                            clipboard,
                            &mut Vec::new(),
                        );

                        if let user_interface::State::Updated {
                            mouse_interaction, ..
                        } = state
                        {
                            window.set_cursor(conversion::mouse_interaction(mouse_interaction));
                        }

                        interface.draw(
                            renderer,
                            &Theme::Ferra,
                            &renderer::Style::default(),
                            self.cursor,
                        );
                        self.cache = interface.into_cache();

                        renderer.present(None, frame.texture.format(), &view, &app_data.viewport);

                        frame.present();
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
    let _ = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(15));
            let _ = new_proxy.send_event(UserEvent::Tick);
        }
    });

    let mut app = App::new(proxy);
    event_loop.run_app(&mut app).expect("Should run event loop");
}
