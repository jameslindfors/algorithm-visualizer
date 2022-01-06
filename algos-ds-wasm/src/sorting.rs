use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn _bubble_sort(array: Vec<i32>) -> Vec<i32> {
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

pub fn new_arr (size: u32) -> Vec<u32> {
    let mut arr: Vec<u32> = Vec::with_capacity(size as usize);
    for _ in 0..size {
        arr.push(rand::thread_rng().gen_range(0, 100));
    }
    arr
}