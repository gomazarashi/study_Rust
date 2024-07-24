fn main() {
    const GIFTS: [&str; 12] = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            DAYS[i]
        );

        if i == 0 {
            println!("{}", GIFTS[i]);
        } else {
            for j in (1..=i).rev() {
                println!("{}", GIFTS[j]);
            }
            println!("and {}", GIFTS[0]);
        }

        println!("----");
    }
}
