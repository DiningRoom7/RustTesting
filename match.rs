
fn main() {
    let str1 = "hello";
    match_str(str1);

    let str2 = "Hello I am string";
    match_str(str2);

    match_int(13);
    match_int(5);
    match_int(29);
}

fn match_str(str: &str) {
    println!("Matching string \"{}\":", str);
    match str {
        "Hello I am string" => println!("Nice"),
        "hi" => println!("Not cool"),
        "6" => println!("Even not cooler"),
        _ => println!("Nicer!")
    }
    println!();
}

fn match_int(number: i32) {
    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // TODO ^ Try adding 13 to the list of prime values
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }
    println!();
}