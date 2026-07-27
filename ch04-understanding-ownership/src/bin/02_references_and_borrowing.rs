fn main() {
    // let s = String::from("hello");
    // change(&s);

    let reference_to_nothing = dangle();
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
