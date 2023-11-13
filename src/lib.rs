use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext;

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");

    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    let dst = doc.get_element_by_id("wasm-example").unwrap();
    let canvas = doc
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    dst.append_child(&canvas).unwrap();

    canvas.set_width(500);
    canvas.set_height(500);
    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();
    webgl2_context.clear_color(0.0, 0.0, 0.4, 0.4);
    webgl2_context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
}
