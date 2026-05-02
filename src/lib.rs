pub mod decode;
pub mod encode;
pub mod game;
pub mod utils;

use wasm_bindgen::prelude::*;

use crate::{
    decode::v1::decode_game,
    encode::v1::{encode_game, encode_puzzle_only, encode_with_solution},
    game::{create_game, hide_answer_mask},
    utils::{Level, grid_to_js, mask_to_js},
};

#[wasm_bindgen]
pub fn new_game_web_v1(n: JsValue, max_try: JsValue, level_js: JsValue) -> JsValue {
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
            let level = Level::from(level_js);
            let mask = hide_answer_mask(n as usize, level);
            let encoded = encode_game(&v, &mask, level);

            let grid_js = grid_to_js(v);
            let mask = mask_to_js(mask);
            JsValue::from(
                [
                    level.as_js_value(),
                    grid_js,
                    mask,
                    JsValue::from_str(encoded[0].as_str()),
                    JsValue::from_str(encoded[1].as_str()),
                ]
                .to_vec(),
            )
        }
        None => JsValue::null(),
    }
}

#[wasm_bindgen]
pub fn decode_game_v1(value: JsValue) -> JsValue {
    let value = match value.as_string() {
        Some(v) => v,
        None => return JsValue::null(),
    };
    match decode_game(value) {
        Some((g, is_hide_ans, table, mask)) => {
            let e1 = encode_puzzle_only(&table, &mask, g);
            let e2 = if is_hide_ans {
                String::new()
            } else {
                encode_with_solution(&table, &mask, g)
            };

            let table = grid_to_js(table);
            let mask = mask_to_js(mask);
            JsValue::from(
                [
                    g.as_js_value(),
                    table,
                    mask,
                    JsValue::from_str(e1.as_str()),
                    if e2.is_empty() {
                        JsValue::null()
                    } else {
                        JsValue::from_str(e2.as_str())
                    },
                ]
                .to_vec(),
            )
        }
        None => JsValue::null(),
    }
}
// size: 9
// new with answer
// v1_10CRI4SWdZhyFFNjchhZZGlHKBU3aDUkkYNUdiGVQWk4J0VpMXgiGVZzSAznbSUiUU22y5hQA=
//
// new without answer
// v1_11CRAASQBQBwAFMDcBAJBGBAIBA3CDUgkINQBgCQAWAIAEBpAAgAEFZzCA
//
// old with answer
// aTF8aTJ8OXxpNnw3fGk4fGk1fDN8aTQtOHxpMXxpNXxpMnxpNHwzfGk2fGk3fGk5LTl8OHwyfDd8NXw2fDN8NHxpMS1pM3w3fGkxfDR8aTZ8OXwyfGk1fGk4LTV8OXwzfGk4fDJ8aTR8aTF8aTZ8aTctNHw1fDd8MXxpM3xpMnw5fDh8aTYtaTJ8aTZ8aTR8aTV8MXw3fGk4fDl8My02fDR8OHxpM3xpOXw1fGk3fGkxfDItN3wzfDZ8aTl8aTh8MXxpNHxpMnw1$eyJsIjoiMiJ9
//
