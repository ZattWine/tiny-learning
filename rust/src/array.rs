// Arrays - fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // re-assign value: need mut
    numbers[2] = 101;

    println!("{:?}", numbers);

    // get single value
    println!("Single val: {}", numbers[0]);

    // array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);
}
