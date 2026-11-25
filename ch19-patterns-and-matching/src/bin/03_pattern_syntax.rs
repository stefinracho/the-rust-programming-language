fn main() {
    enum Message {
        Quit,
        WriteString(String),
        Move { x: i32, y: i32 },
        ChangeColor(u8, u8, u8),
    }
    let message = Message::Quit;
    match message {
        Message::Quit => println!("Quit"),
        Message::WriteString(write) => println!("{}", &write),
        Message::Move { x, y: 0 } => println!("move {} horizontally", x),
        Message::Move { .. } => println!("other move"),
        Message::ChangeColor {
            0: red,
            1: green,
            2: _,
        } => {
            println!("color change, red: {}, green: {}", red, green);
        }
    };

    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("{feet}, {inches}, {x}, {y}");

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");
    match s {
        Some(ref x) => println!("some"),
        None => println!("None"),
    }
    println!("{s:?}");

    // let words = vec!["a", "b", "c"];
    // let slice = &words[..];
    // match slice {
    //     [] => println!("slice is empty"),
    //     [one] => println!("single element {}", one),
    //     [head, tail @ ..] => println!("head={} tail={:?}", head, tail),
    // }

    let num = 0;

    match num {
        x if x % 2 == 0 => println!("The number {x} is even"),
        x => println!("The number {x} is odd"),
    }

    let int_reference = &3;

    let a = match *int_reference {
        0 => "zero",
        _ => "some",
    };
    let b = match int_reference {
        &0 => "zero",
        _ => "some",
    };
    println!("{a}, {b}");

    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {id}"),
    }
}
