fn main() {
    assert_eq!(pig_latin("first"), "irst-fay");
    assert_eq!(pig_latin("banana"), "anana-bay");
    assert_eq!(pig_latin("world"), "orld-way");

    assert_eq!(pig_latin("apple"), "apple-hay");
    assert_eq!(pig_latin("ink"), "ink-hay");
    assert_eq!(pig_latin("under"), "under-hay");

    assert_eq!(pig_latin("a"), "a-hay");
    assert_eq!(pig_latin("i"), "i-hay");
    assert_eq!(pig_latin("b"), "b-ay");

    assert_eq!(pig_latin("123"), "123");
    assert_eq!(pig_latin("!hello"), "!hello");

    assert_eq!(pig_latin(""), "");
}

// Only works for unaccented English alphabet
fn pig_latin(s: &str) -> String {
    let first_char = match s.chars().next() {
        Some(c) => c,
        None => return String::new(),
    };

    let vowels = String::from("aAeEiIoOuU");
    let consonants = String::from("bBcCdDfFgGhHjJkKlLmMnNpPqQrRsStTvVwWxXyYzZ");
    let is_vowel = vowels.contains(first_char);
    let is_consonant = consonants.contains(first_char);

    if is_vowel {
        format!("{s}-hay")
    } else if is_consonant && s.len() > 1 {
        format!("{}-{first_char}ay", &s[1..])
    } else if is_consonant && s.len() == 1 {
        format!("{first_char}-ay")
    } else {
        s.to_string()
    }
}
