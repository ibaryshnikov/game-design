use std::cell::RefCell;
use std::ops::Drop;
use std::rc::Rc;

use bytes::Bytes;
use js_sys::Date;
use nalgebra::Point2;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent, Window};

use game_core::boss::Boss;
use game_core::hero::Hero;
use game_core::scene::{self, Scene};
use network::client::{self, KeyActionKind, Move};
use network::server;

use crate::dom_helpers::*;
use crate::hero::HeroView;
use crate::scene::SceneView;

#[derive(Default)]
struct Callbacks {
    onkeydown: Option<Closure<dyn FnMut(KeyboardEvent)>>,
    onkeyup: Option<Closure<dyn FnMut(KeyboardEvent)>>,
}

pub struct Stage {
    ws_write: js_sys::Function,
    width: u16,
    height: u16,
    window: Window,
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    callbacks: Callbacks,
    hero: Hero,
    scene: Scene,
    last_update: u128,
    pub state_changed: bool,
}

impl Stage {
    pub fn new(ws_write: js_sys::Function) -> Result<Stage, JsValue> {
        let window = get_window()?;
        let document = get_document(&window)?;
        let body = get_body(&document)?;

        let width = 1000;
        let height = 750;

        let canvas = create_canvas(&document)?;

        canvas.set_width(u32::from(width));
        canvas.set_height(u32::from(height));

        body.append_child(&canvas)?;

        let ctx = get_context(&canvas)?;

        let tmp_id = 0; // will receive a proper one from server when connected
        let hero = Hero::new(tmp_id, Point2::new(250.0, 200.0));
        let scene = Scene::new(scene::Mode::Client);

        Ok(Stage {
            ws_write,
            width,
            height,
            window,
            canvas,
            ctx,
            callbacks: Callbacks::default(),
            hero,
            scene,
            last_update: Date::now() as u128,
            state_changed: true,
        })
    }

    pub fn add_listeners(&mut self, self_ref: Rc<RefCell<Stage>>) {
        console_log!("Adding listeners");
        self.set_keydown(self_ref.clone());
        self.set_keyup(self_ref);
    }

    pub fn remove_listeners(&mut self) {
        self.window.set_onkeydown(None);
        self.window.set_onkeyup(None);
        self.callbacks.onkeydown = None;
        self.callbacks.onkeyup = None;
    }

    fn set_keydown(&mut self, self_ref: Rc<RefCell<Stage>>) {
        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            self_ref
                .borrow_mut()
                .process_key_code(&e.code(), KeyActionKind::Pressed);
        }) as Box<dyn FnMut(KeyboardEvent)>);
        self.window
            .set_onkeydown(Some(closure.as_ref().unchecked_ref()));
        self.callbacks.onkeydown = Some(closure);
    }
    fn set_keyup(&mut self, self_ref: Rc<RefCell<Stage>>) {
        let closure = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            self_ref
                .borrow_mut()
                .process_key_code(&e.code(), KeyActionKind::Released);
        }) as Box<dyn FnMut(KeyboardEvent)>);
        self.window
            .set_onkeyup(Some(closure.as_ref().unchecked_ref()));
        self.callbacks.onkeyup = Some(closure);
    }

    fn process_key_code(&mut self, code: &str, kind: KeyActionKind) {
        let action = match code {
            "KeyW" => Move::Up,
            "KeyS" => Move::Down,
            "KeyA" => Move::Left,
            "KeyD" => Move::Right,
            "ShiftLeft" | "ShiftRight" => {
                if let KeyActionKind::Pressed = kind {
                    // return ui_app::Message::HeroDash;
                }
                return;
                // return ui_app::Message::None;
            }
            "Space" => {
                if let KeyActionKind::Pressed = kind {
                    // return ui_app::Message::HeroAttack;
                }
                return;
                // return ui_app::Message::None;
            }
            _ => {
                return;
                //return ui_app::Message::None,
            }
        };
        self.handle_move_action(kind, action);
        // let kind = match event.state {
        //     ElementState::Pressed => client::KeyActionKind::Pressed,
        //     ElementState::Released => client::KeyActionKind::Released,
        // };
        //
        // match (code, direction) {
        //     ("ArrowLeft", Down) => {
        //         console_log!("ArrowLeft pressed");
        //     }
        //     ("ArrowRight", Down) => {
        //         console_log!("ArrowRight pressed");
        //     }
        //     ("ArrowLeft", Up) => {
        //         console_log!("ArrowLeft released");
        //     }
        //     ("ArrowRight", Up) => {
        //         console_log!("ArrowRight released");
        //     }
        //     ("Space", Down) => {
        //         console_log!("Space pressed");
        //     }
        //     _ => (),
        // }
    }

    fn handle_move_action(&mut self, kind: KeyActionKind, movement: Move) {
        self.hero.handle_move_action(kind.clone(), movement.clone());
        let message = client::Message::Move(kind, movement);
        let bytes = Bytes::from(message.to_vec());
        self.send_to_server(&bytes);
        self.state_changed = true;
    }

    fn send_to_server(&self, data: &[u8]) {
        let array = js_sys::Uint8Array::from(data);
        let argument1 = &JsValue::from(array);
        let _ = self.ws_write.call1(&JsValue::NULL, argument1);
    }

    pub fn update_state(&mut self) {
        let now = Date::now() as u128;
        let dt = now - self.last_update;
        self.last_update = now;
        self.hero.update_visuals(dt);
        self.scene.update(dt);
        self.state_changed = true;
    }

    pub fn handle_server_message(&mut self, message: server::Message) {
        match message {
            server::Message::Test => {
                console_log!("Got server::Message::Test");
            }
            server::Message::SetId(id) => {
                console_log!("Got id from server: {}", id);
                self.hero.id = id;
            }
            server::Message::Update(update) => {
                console_log!("Got Update message from server");
                // self.scene.handle_server_update(update);
                match update {
                    server::Update::Scene(scene) => {
                        console_log!("Got Scene update from server");
                        for (key, network_character) in scene.characters.into_iter() {
                            console_log!("Network character {:?}", network_character);
                            let character = Hero::from_network(network_character);
                            if character.id == self.hero.id {
                                self.hero.position = character.position;
                            } else {
                                self.scene.characters.insert(key, character);
                            }
                        }
                        console_log!("New position: {:?}", self.hero.position);
                        self.scene.npc = scene.npc.into_iter().map(Boss::from_network).collect();
                    }
                    other => {
                        console_log!("Got some other update: {:?}", other);
                    }
                }
                self.state_changed = true;
            }
        }
    }

    pub fn draw(&self) {
        if !self.state_changed {
            return;
        }
        self.ctx
            .clear_rect(0.0, 0.0, f64::from(self.width), f64::from(self.height));

        HeroView::new(&self.hero).draw(&self.ctx);
        SceneView::new(&self.scene).draw(&self.ctx);
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        self.canvas.remove();
    }
}
