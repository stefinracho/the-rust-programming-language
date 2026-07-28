#[derive(Debug)]
struct Rectange {
    width: u32,
    height: u32,
}

fn main() {
    // let scale = 2;
    let rect1 = Rectange {
        width: 30,
        height: 50,
    };

    // println!(
    //     "The area of the rectange is {} square pixels.",
    //     area(&rect1)
    // );
    //
    // println!("rect1 is {rect1:?}");

    dbg!(&rect1);
}

// fn area(rectange: &Rectange) -> u32 {
//     rectange.width * rectange.height
// }
