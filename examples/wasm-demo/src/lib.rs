use bezier_easing::{BezierEasing, bezier_easing};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmBezierEasing {
    easing: BezierEasing,
}

#[wasm_bindgen]
impl WasmBezierEasing {
    #[wasm_bindgen(constructor)]
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Result<WasmBezierEasing, JsValue> {
        let easing = bezier_easing(x1, y1, x2, y2).map_err(|error| error.to_string())?;

        Ok(Self { easing })
    }

    pub fn sample(&self, x: f64) -> f64 {
        (self.easing)(x)
    }
}
