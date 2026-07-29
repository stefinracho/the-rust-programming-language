// enum IpAddrKind {
//     V4,
//     V6,
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    //
    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
}

// fn route(ip_kind: IpAddrKind) {}
