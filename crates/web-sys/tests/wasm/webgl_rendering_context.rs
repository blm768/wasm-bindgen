use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

#[wasm_bindgen(module = "./tests/wasm/webgl.js")]
extern {
    fn new_webgl_rendering_context() -> WebGlRenderingContext;
}

#[wasm_bindgen_test]
fn test_webgl_rendering_context() {
    let context = new_webgl_rendering_context();
}
