mod canvas;
mod sorting;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let arr: Vec<u32> = sorting::new_arr(50);
    let bars_arr = canvas::gen_bar_arr(arr.clone());
    canvas::draw_bars(bars_arr);

    utils::log_vec(arr);

    utils::log("Start Executed");
    Ok(())
}