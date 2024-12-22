fn main() {
    let days =["firs", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    let gifts = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese-a-laying",
        "Seven swans-a-swimming",
        "Eight maids-a-milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve lords-a-leaping"
    ];
    for i in 0..12{
        println!("On the {} day of Christmas", days[i]);
        println!("My true love gave to me");
        for j in (0..=i).rev() {
            if i > 0 && j == 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }  
        println!();
    }
}