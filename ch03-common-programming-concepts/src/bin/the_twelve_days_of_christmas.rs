fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four colly birds,",
        "Five gold rings,",
        "Six geese a laying,",
        "Seven swans a swimming,",
        "Eight maids a milking,",
        "Nine drummers drumming,",
        "Ten pipers piping,",
        "Eleven lading dancing,",
        "Twelve lords a leaping,",
    ];
    for (day_idx, day) in days.iter().enumerate() {
        println!("The {day} day of Christmas,");
        println!("My true love sent to me");
        for &gift in gifts[..=day_idx].iter().rev() {
            if gift == "Two turtle doves," {
                println!("Two turtle doves, and");
            } else {
                println!("{gift}");
            }
        }
        println!();
    }
}
