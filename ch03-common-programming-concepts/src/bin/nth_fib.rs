use std::io::{Write, stdin, stdout};

fn main() {
    println!("Generate the nth Fibonacci number");
    print!("n: ");
    stdout().flush().expect("stdout should flush");
    let mut n = String::new();
    stdin().read_line(&mut n).expect("stdin should read line");
    let n = n.trim().parse().expect("n should be number type");
    println!("The {n}th Fibonacci number is: {}", fib(n));
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
