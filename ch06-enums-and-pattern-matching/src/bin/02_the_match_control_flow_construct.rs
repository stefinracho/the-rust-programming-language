// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

fn main() {
    // let value = value_in_cents(Coin::Quarter(UsState::Alaska));
    // println!("value = {value}");

    let five = Some(5);
    let six = plus_one(five);
    println!("six = {six:?}");
    let none = plus_one(None);
    println!("none = {none:?}");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {state:?}!");
//             25
//         }
//     }
// }
