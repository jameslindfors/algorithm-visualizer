mod utils;
mod sorting;

use wasm_bindgen::prelude::*;

// Utils

// Sorting
#[wasm_bindgen]
pub fn bubble_sort(array: Vec<i32>) -> Vec<i32> {
    sorting::bubble_sort(array)
}