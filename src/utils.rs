use js_sys::Array;
use wasm_bindgen::JsValue;

pub(crate) enum GemeType {
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

impl GemeType {
    pub(crate) fn to_js_value(&self) -> JsValue {
        match self {
            Self::Easy => JsValue::from_str("easy"),
            Self::Medium => JsValue::from_str("medium"),
            Self::Difficult => JsValue::from_str("difficult"),
        }
    }
}

pub(crate) fn create_game_v1(value: Vec<Vec<u8>>, game_level: GemeType) -> JsValue {
    let c = value
        .into_iter()
        .map(|v| JsValue::from(v))
        .collect::<Vec<JsValue>>();
    let array = Array::new();
    for raw in c {
        array.push(&raw);
    }
    let table = JsValue::from(array);
    todo!()
}

