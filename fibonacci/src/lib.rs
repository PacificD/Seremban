use std::io;

pub fn generate() {
    println!("Please enter a number");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Failed to parse number");

    println!("The {n} fibonacci number is {}", fibonacci(n))
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 0;
    }

    if n == 2 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}
