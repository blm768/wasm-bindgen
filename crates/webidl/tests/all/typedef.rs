use super::project;

#[test]
fn typedef() {
    project()
        .file(
            "typedef.webidl",
            r#"
                typedef long MyLong;
            "#,
        )
        .file(
            "src/lib.rs",
            r#"
                #![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]
                extern crate wasm_bindgen;
                use wasm_bindgen::prelude::*;

                pub mod typedef;
                use typedef::MyLong;

                #[wasm_bindgen]
                pub fn test() {
                }
            "#,
        )
        .test();
}
