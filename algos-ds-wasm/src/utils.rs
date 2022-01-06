use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn _window () -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn _request_animation_frame(f: &Closure<dyn FnMut()>) {
    _window().request_animation_frame(f.as_ref().unchecked_ref()).expect("should register `requestAnimationFrame` OK");
}

pub fn document() -> web_sys::Document {
    _window()
        .document()
        .expect("should have a document on the window")
} 

pub fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

pub fn canvas() -> web_sys::HtmlCanvasElement{
    let canvas = document().get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
}

pub fn ctx () -> web_sys::CanvasRenderingContext2d {
    let canvas = canvas();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_i32(a: i32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_many(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_vec(a: Vec<u32>);
}