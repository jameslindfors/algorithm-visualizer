#[path = "utils.rs"]
mod utils;

use std::f64;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// Create Bar struct
pub struct Bar {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color: String,
}

impl Bar {
    fn new(x: f64, y: f64, width: f64, height: f64, color: String) -> Bar {
        Bar {
            x: x,
            y: y,
            width: width,
            height: height,
            color: color,
        }
    }  
    fn draw(&self) {
        utils::ctx().begin_path();
        utils::ctx().set_fill_style(&JsValue::from_str(&self.color));
        utils::ctx().fill_rect(self.x, self.y, self.width, self.height);
        utils::ctx().fill();
    }
}

// Object drawing
pub fn gen_bar_arr(arr: Vec<u32>) -> Vec<Bar> {
    let mut bars: Vec<Bar> = Vec::with_capacity(arr.len());
    let mut x = 0.0;
    let y = 0.0;
    let width = utils::canvas().width() as f64 / arr.len() as f64;
    let height = utils::canvas().height() as f64 / 100.0;

    for i in 0..arr.len() {
        bars.push(Bar::new(
            x,
            y,
            width,
            arr[i] as f64 * height,
            "black".to_string(),
        ));
        x += width;
    }
    bars
}

pub fn draw_bars (bars: Vec<Bar>) {
    for i in 0..bars.len() {
        bars[i].draw();
    }
}

// Animation Loop
#[wasm_bindgen]
pub fn _run_animation(_arr: Vec<u32>) -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut i = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if i > 350 {
            // utils::body().set_text_content(Some("All Done!"));
            utils::log("All Done!");
            // Drop handle to clean
            let _ = f.borrow_mut().take();
            return;
        }
        i += 1;

    let _text = format!("requestAnimationFrame has been called {} times", i);
    // utils::log(&_text as &str);

    // Schedule next frame
    utils::_request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    utils::_request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

// Canvas Utils
#[wasm_bindgen]
pub fn _set_canvas_size(width: u32, height: u32) {
    let canvas = utils::canvas();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
}