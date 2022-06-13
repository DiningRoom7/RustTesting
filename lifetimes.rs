

fn main() {
    //println!("{}", longest("abab", "aaaabbbb"));

    let o: &str;
    
    {
        let b = "aaabbb";
        let a = "abab";

        o = longest(a, b);
    }
    println!("{o}");
}

//This function does not work without generic lifetimes
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}