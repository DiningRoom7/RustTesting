fn main() {
    let s1 = String::from("I am string.");

    foo(&s1);

    //s1 could no longer be referenced here if not passed as reference
    println!("s1 is still valid, see: \"{s1}\"");
}

//If s was not a reference, s would change ownership
//and its scope would end at the end of foo
fn foo(s: &String) {
    println!("The value of s is \"{s}\"");
}