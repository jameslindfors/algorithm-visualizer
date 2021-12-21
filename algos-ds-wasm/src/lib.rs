mod canvas;
mod sorting;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    canvas::draw_circle(40.0);
    canvas::set_canvas_size(1000.0, 350.0);
    canvas::run_animation().unwrap();
    sorting::new_arr(5);
    utils::log("Start Executed");

}