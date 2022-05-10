
fn main() {
    let str1 = "hello";
    match_str(str1);

    let str2 = "Hello I am string";
    match_str(str2);

    let str3 = "hi";
    match_str(str3);
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