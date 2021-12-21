mod canvas;
mod sorting;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let mut arr: Vec<i32>= Vec::new();
    let mut bars: Vec<canvas::Bar> = Vec::new();

    canvas::set_canvas_size(1000.0, 350.0);
    sorting::new_arr(1000, &mut arr);
    canvas::create_bar_arr(arr, &mut bars);
    canvas::draw_bars(bars);

    utils::log("Start Executed");

}