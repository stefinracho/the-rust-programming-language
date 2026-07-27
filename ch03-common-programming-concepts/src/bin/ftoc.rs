use std::io::{self, Write, stdin};

fn main() {
    println!("Fahrenheit to Celsius");
    print!("Fahrenheit: ");
    io::stdout().flush().expect("stdout should flush");
    let mut f = String::new();
    stdin().read_line(&mut f).expect("stdin should read line");
    let f: f64 = f.trim().parse().expect("Fahrenheit should be number type");
    let c = (f - 32.0) * 5.0 / 9.0;
    println!("Celsius: {c:.2}");
}
