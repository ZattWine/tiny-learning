// Reference Pointers - Point to a resource in memory

pub fn run() {
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    // With non-primitives, if you assign another variable to a piece of data,
    // the first variable no longer hold that value.
    // You'll need to use a reference (&) to point to the resource.
    let vec1: Vec<i32> = vec![1, 2, 3, 4];
    let vec2 = &vec1;

    println!("Values: {:?}", (arr1, arr2));
    println!("Values: {:?}", (&vec1, vec2));
}
