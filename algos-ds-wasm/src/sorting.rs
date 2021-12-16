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
