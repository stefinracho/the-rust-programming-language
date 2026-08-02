fn main() {
    // let v: Vec<i32> = Vec::new();

    // Updating a Vector
    // let mut v = vec![1, 2, 3];
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // Reading Elements of Vectors
    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // println!("The third element is {third}");
    //
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }
    //
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
    //
    // Iterating Over the values in a Vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
}
