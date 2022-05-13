fn main() {
    let s1 = String::from("I am string.");

    foo(&s1);

    println!("s1 is still valid, see: \"{s1}\"");
}

fn foo(s: &String) {
    println!("The value of s is \"{s}\"");
}