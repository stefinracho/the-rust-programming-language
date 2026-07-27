fn main() {
    let guess = "42".parse().expect("Not a number!");

    let tup = ("hello", 5, 'c');
    let (hello, five, c) = tup; // destructuring
    println!("{hello}, {five}, {c}");
}
