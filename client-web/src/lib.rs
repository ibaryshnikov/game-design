use std::cell::RefCell;
use std::ops::Drop;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use network::server;

#[macro_use]
mod console;

mod attack;
mod boss;
mod dom_helpers;
mod hero;
mod scene;
mod stage;

use stage::Stage;

#[wasm_bindgen]
pub struct StageContainer {
    stage: Rc<RefCell<Stage>>,
}

#[wasm_bindgen]
impl StageContainer {
    #[wasm_bindgen(constructor)]
    pub fn new(ws_write: js_sys::Function) -> Result<StageContainer, JsValue> {
        console_error_panic_hook::set_once();

        let stage = Rc::new(RefCell::new(Stage::new(ws_write)?));

        stage.borrow_mut().add_listeners(stage.clone());

        Ok(StageContainer { stage })
    }

    #[wasm_bindgen(js_name = updateState)]
    pub fn update_state(&mut self) {
        self.stage.borrow_mut().update_state();
    }

    #[wasm_bindgen(js_name = handleWsMessage)]
    pub fn handle_ws_message(&mut self, data: Vec<u8>) {
        // console_log!("got ws message in wasm, len {}", data.len());
        let message = server::Message::from_slice(&data);
        // console_log!("Server message is {:?}", message);
        self.stage.borrow_mut().handle_server_message(message);
    }

    pub fn draw(&self) {
        self.stage.borrow().draw();
        self.stage.borrow_mut().state_changed = false;
    }
}

impl Drop for StageContainer {
    fn drop(&mut self) {
        console_log!(
            "removing listeners, reference count {}",
            Rc::strong_count(&self.stage)
        );
        self.stage.borrow_mut().remove_listeners();
        console_log!("done, reference count {}", Rc::strong_count(&self.stage));
    }
}
