fn main() {
    let mut s1 = String::from("I am string.");
    //  ^^^ <-- s1 must be mut in the first place

    foo(&mut s1);

    println!("{s1}");
}

fn foo(s: &mut String) {
    //    ^^^^ <-- mut reference applied to type
    println!("Changing s1 from foo!");
    s.push_str(" hi!");
}