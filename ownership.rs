fn main() {
    let s1 = String::from("I am string.");

    foo(s1);

    //This does not work:
    //  println!("{s1}");
    
    //s1's ownership was transfered to foo and
    //went out of scope at the end of foo
}

fn foo(s: String) {
    println!("{s}");
}