fn main() {
    let s1 = String::from("I am s1");
    let _s2 = s1;
    //This does not work:
    //  println!("{s1}");
    //s1's ownership was transfered to s2

    let s3 = String::from("I am string.");
    foo(s3);
    //This does not work:
    //  println!("{s3}");
    //s3's ownership was transfered to foo and
    //went out of scope at the end of foo


    let n1 = 32;
    bar(n1);
    println!("{n1} from main");
    //This is okay because ints, floats, bools, and chars
    //implement the copy trait
}

fn foo(s: String) {
    println!("{s} from foo");
}

fn bar(n: i32) {
    println!("{n} from bar");
}