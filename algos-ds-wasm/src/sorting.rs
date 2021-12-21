use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bubble_sort(array: Vec<i32>) -> Vec<i32> {
    let mut array = array;
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }
    }
    array
}

pub fn new_arr (size: i32, arr: &mut Vec<i32>) {
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        arr.push(rng.gen_range(0, size));
    }
}