use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    Easy,
    Medium,
    Difficult,
}

impl Level {
    pub(crate) fn as_js_value(&self) -> JsValue {
        match self {
            Self::Easy => JsValue::from_str("Easy"),
            Self::Medium => JsValue::from_str("Medium"),
            Self::Difficult => JsValue::from_str("Difficult"),
        }
    }
}

impl From<JsValue> for Level {
    fn from(value: JsValue) -> Self {
        let v = value.clone().as_string().map(|v| v.to_ascii_lowercase());
        match v.as_deref() {
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

pub(crate) fn grid_to_js(value: Vec<Vec<u8>>) -> JsValue {
    let v = value
        .into_iter()
        .map(JsValue::from)
        .collect::<Vec<JsValue>>();
    JsValue::from(v)
}

pub(crate) fn mask_to_js(mask: Vec<Vec<bool>>) -> JsValue {
    let mask = mask
        .into_iter()
        .map(|v| {
            JsValue::from(
                v.into_iter()
                    .map(JsValue::from_bool)
                    .collect::<Vec<JsValue>>(),
            )
        })
        .collect::<Vec<JsValue>>();
    JsValue::from(mask)
}

pub(crate) fn full_mask_from_table(v: &[Vec<u8>]) -> Vec<Vec<bool>> {
    v.iter()
        .map(|m| m.iter().map(|&n| n == 0).collect())
        .collect()
}
