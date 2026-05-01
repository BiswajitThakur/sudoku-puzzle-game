pub(crate) mod decode;
pub mod encode;
pub mod game;
pub mod utils;

use wasm_bindgen::prelude::*;

use crate::{
    game::{create_game, hide_answer_mask},
    utils::{GemeType, vec_vec_bool_to_js_value, vec_vecu8_to_js_value_v1},
};

#[wasm_bindgen]
pub fn create_game_web(n: JsValue, max_try: JsValue, t: JsValue) -> JsValue {
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
        Some(v) => {
            let t = GemeType::from(t);
            let mask = hide_answer_mask(n as usize, t);

            let e1 = encode_with_answer(&v, &mask, t);
            let e2 = encode_without_answer(&v, &mask, t);

            let g = vec_vecu8_to_js_value_v1(v);
            let mask = vec_vec_bool_to_js_value(mask);
            JsValue::from([g, mask, e1, e2].to_vec())
        }
        None => JsValue::null(),
    }
}

pub fn encode_with_answer(
    v: &Vec<Vec<u8>>,
    mask: &Vec<Vec<bool>>,
    game_level: GemeType,
) -> JsValue {
    todo!()
}

pub fn encode_without_answer(
    v: &Vec<Vec<u8>>,
    mask: &Vec<Vec<bool>>,
    game_level: GemeType,
) -> JsValue {
    todo!()
}
