

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
//Error message: "this function's return type contains a borrowed value,
//                but the signature does not say whether it is borrowed 
//                from `s1` or `s2`"
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    //    ^^^^------^^-----------^^----------^^---> These lifetime specifications must be added
    //This just means it will find some lifetime such that s1 and s2 will live at least that long
    //Note that the lifetime of the return value must be the same as one of the parameters
    if s1.len() > s2.len() { s1 } else { s2 }
}