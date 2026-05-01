use wasm_bindgen::JsValue;

use crate::game::hide_answer_mask;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GemeType {
    Easy,
    Medium,
    Difficult,
}

impl From<JsValue> for GemeType {
    fn from(value: JsValue) -> Self {
        let v = value.clone().as_string().map(|v| v.to_ascii_lowercase());
        match v.as_ref().map(|v| v.as_str()) {
            Some("easy") => return Self::Easy,
            Some("medium") => return Self::Medium,
            Some("difficult") => return Self::Difficult,
            _ => {}
        }
        match value.as_f64().map(|v| v as i32) {
            Some(0) => Self::Easy,
            Some(1) => Self::Medium,
            _ => Self::Difficult,
        }
    }
}

pub(crate) fn vec_vecu8_to_js_value_v1(value: Vec<Vec<u8>>) -> JsValue {
    let v = value
        .into_iter()
        .map(|v| JsValue::from(v))
        .collect::<Vec<JsValue>>();
    JsValue::from(v)
}

pub(crate) fn vec_vec_bool_to_js_value(mask: Vec<Vec<bool>>) -> JsValue {
    let mask = mask
        .into_iter()
        .map(|v| {
            JsValue::from(
                v.into_iter()
                    .map(|b| JsValue::from_bool(b))
                    .collect::<Vec<JsValue>>(),
            )
        })
        .collect::<Vec<JsValue>>();
    JsValue::from(mask)
}
