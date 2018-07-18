import * as wasm from "./out";

export function new_webgl_rendering_context() {
    let canvas = document.createElement("canvas");
    document.body.appendChild(canvas);
    let context = canvas.getContext("webgl");
    wasm.test_webgl_rendering_context(context);
}
