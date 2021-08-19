pub fn run() {
    greeting("Hello", "Kyaw");

    // bind function values to variable
    let get_sum = add(1, 5);
    println!("Sum: {}", get_sum);

    // closure
    let n3: i32 = 3;
    let add_sums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_sums(3, 4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you.", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
