fn main() {
    let mut s1 = String::from("I am string.");
    //  ^^^ <-- s1 must be mut in the first place

    foo(&mut s1);

    println!("{s1}");

    //NOTE THAT TWO MUT REFS CANNOT BE CREATED FOR THE SAME VARIABLE
    //ONE MUT REF PER VARIABLE
    //This does not work:
    //  let s1_mutref1 = &mut s1;
    //  let s1_mutref2 = &mut s1;
    //  println!("{s1_mutref1} {s1_mutref2}");
}

fn foo(s: &mut String) {
    //    ^^^^ <-- mut reference applied to type
    println!("Changing s1 from foo!");
    s.push_str(" hi!");
}