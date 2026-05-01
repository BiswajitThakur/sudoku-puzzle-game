pub(crate) mod decode;
pub(crate) mod encode;
pub(crate) mod game;
pub(crate) mod utils;

use wasm_bindgen::prelude::*;

use crate::{
    game::create_game,
    utils::{GemeType, create_game_v1},
};

#[wasm_bindgen]
pub fn create_game_web(n: JsValue, max_try: JsValue, game_level: JsValue) -> JsValue {
    let n = match n.as_f64() {
        Some(v) => {
            let v = v as usize;
            if v > 14 {
                return JsValue::null();
            }
            v as u8
        }
        None => return JsValue::null(),
    };
    let max_try = match max_try.as_f64() {
        Some(v) => v as usize,
        None => return JsValue::null(),
    };
    match create_game(n, max_try) {
        Some(v) => create_game_v1(v, GemeType::from(game_level)),
        None => return JsValue::null(),
    }
}

#[wasm_bindgen]
pub fn encode_with_answer(game: JsValue, game_level: JsValue) -> JsValue {
    todo!()
}

#[wasm_bindgen]
pub fn encode_without_answer(game: JsValue, game_level: JsValue) -> JsValue {
    todo!()
}
