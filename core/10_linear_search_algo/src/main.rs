fn linear_search(arr: &mut [i32], _value: i32) {
    let mut is_found:bool = false;
    for x in 0..arr.len() {
        if arr[x] == _value {
            println!("Index of {} : {}",_value, x);
            is_found = true;
            break;
        }
    }
    if !is_found {
        println!("{} does not exist in the array {:?}",_value,arr)
    }
}

fn main() {
    let mut _arr: [i32; 5] = [1, 2, 3, 4, 5];
    linear_search(&mut _arr, 6);
}
