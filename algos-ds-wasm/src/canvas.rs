#[path = "utils.rs"]
mod utils;

use std::f64;
use std::cell::RefCell;
use std::rc::Rc;
use rand::Rng;
use wasm_bindgen::prelude::*;

// Object drawing
#[wasm_bindgen]
pub fn draw_circle(radius: f64) {
    let color = gen_rand_hex();
    let cords = gen_rand_cords();

    utils::ctx().begin_path();
    utils::ctx().arc(cords.0, cords.1, radius, 0.0, f64::consts::PI * 2.0)
        .unwrap();
    utils::ctx().set_fill_style(&JsValue::from_str(&color));
    utils::ctx().fill();
}

// Animation Loop
#[wasm_bindgen]
pub fn run_animation() -> Result<(), JsValue> {
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
        // Line that renders new circle each iteration 
        draw_circle(i as f64); 
    let text = format!("requestAnimationFrame has been called {} times", i);

    // Schedule next frame
    utils::request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    utils::request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

// Canvas Utils
#[wasm_bindgen]
pub fn set_canvas_size(width: f64, height: f64) {
    let canvas = utils::canvas();
    canvas.set_width(width as u32);
    canvas.set_height(height as u32);
}
pub fn get_canvas_size() -> (f64, f64) {
    let canvas = utils::canvas();
    let width = canvas.width() as f64;
    let height = canvas.height() as f64;
    // utils::log(&format!("canvas width: {}, height: {}", width, height));
    (width, height)
}

fn gen_rand_hex()-> String{
    let mut rng = rand::thread_rng();
    let color = format!("#{:06x}", rng.gen::<u32>());
    // utils::log(&format!("random color: {}", color));
    color
}

fn gen_rand_cords() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0, utils::canvas().width() as f64);
    let y = rng.gen_range(0.0, utils::canvas().height() as f64);
    // utils::log(&format!("x: {}, y: {}", x, y));
    (x, y)
}
