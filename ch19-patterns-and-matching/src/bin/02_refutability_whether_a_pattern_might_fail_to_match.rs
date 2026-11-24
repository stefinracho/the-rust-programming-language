fn main() {
    let some_option_value: Option<i32> = None;
    // let Some(x) = some_option_value else {
    //     return;
    // };

    match some_option_value {
        Some(_) => println!("hello"),
        None => println!("None"),
    }

    let dish = ("Ham", "Eggs");
    // This body will be skipped because the pattern is refuted.
    if let ("Bacon", b) = dish {
        println!("Bacon is served with {}", b);
    } else {
        println!("Nothing");
    }

    if let ("Ham", b) = dish {
        println!("Ham is served with {}", b);
    }

    if let _ = 5 {
        println!("Irrefutable patterns are always true");
    }
}
