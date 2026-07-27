fn main() {
    let s1 = String::from("hello world");
    let s2 = "hello world";
    let word1 = first_word(&s1);
    let word2 = first_word(s2);
    println!("the first word is: {word1}");
    println!("the first word is: {word2}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    s
}
