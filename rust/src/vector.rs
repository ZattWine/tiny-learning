// Vectors - resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // re-assign value: need mut
    numbers[2] = 101;

    // add on to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // get single value
    println!("Single val: {}", numbers[0]);

    // vector length
    println!("Vector Length: {}", numbers.len());

    // vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Slice: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop in mutate values
    for x in numbers.iter_mut() {
        *x *= 2
    }
    println!("Numbers Vec: {:?}", numbers);
}
