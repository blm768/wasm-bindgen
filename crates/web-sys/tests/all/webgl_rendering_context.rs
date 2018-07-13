use super::websys_project;

#[test]
fn webgl_rendering_context() {
    websys_project()
        .file(
            "src/lib.rs",
            r#"
                #![feature(proc_macro, wasm_custom_section)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;
                extern crate web_sys;

                #[wasm_bindgen]
                pub fn test_webgl_rendering_context(context: WebGLRenderingContext) {
                }
            "#,
        )
        .file(
            "test.js",
            r#"
                import * as assert from "assert";
                import * as wasm from "./out";

                export function test() {
                    let canvas = document.body.createElement("canvas");
                    let context = canvas.context("webgl");
                    wasm.test_content(context);
                }
            "#,
        )
        .test();
}
