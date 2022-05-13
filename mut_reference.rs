fn foo(s: &mut String) {
    //    ^^^^ <-- mut reference applied to type
    println!("Changing s1 from foo!");
    s.push_str(" hi!");
}

fn main() {
    let mut s1 = String::from("I am string.");
    //  ^^^ <-- s1 must be mut in the first place

    foo(&mut s1);

    println!("{s1}");

    //NOTE THAT TWO MUT REFS CANNOT BE CREATED FOR THE SAME VARIABLE 
    //ONE MUT REF PER VARIABLE IN THE SAME SCOPE
    //This does not work:
    //  let s1_mutref1 = &mut s1;
    //  let s1_mutref2 = &mut s1;

    //Instead use curly brackets to limit scope like this:
    //  { let s1_mutref1 = &mut s1; }
    //  let s1_mutref2 = &mut s1;

    //Additionally, mutable references cannot exist in
    //the same scope as immutable ones.
    //This does not work:
    //  let s1_ref = &s1;
    //  let s1_mutref = &mut s1;

    //As many immutable references as you want can be created
    //because this has no datarace potential.
}