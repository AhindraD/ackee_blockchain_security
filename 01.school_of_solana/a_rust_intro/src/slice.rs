fn main() {
    let my_arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("no of elems in arr: {}", my_arr.len());

    let arr_slice = &my_arr[1..4];
    println!("slice of array: {:?}", arr_slice);
}
