//Slices let you reference a contiguous sequence of elements in
//a collection rather than the whole collection.
//A slice is a kind of reference, so it does not have ownership.

fn main() {
    let s1 = String::from("Get words in this string.");
    
    let first = get_first_word(&s1);
    println!("{first}");

    let second = get_second_word(&s1);
    println!("{second}");
}

//takes a reference string slice and returns a reference string slice
fn get_first_word(s: &str) -> &str {
    let chars = s.chars().enumerate();

    for (i, item) in chars {
        if item == ' ' {return &s[..i]}
    }
    return &s[..];
}

fn get_second_word(s: &str) -> &str {
    let chars = s.chars().enumerate();

    let mut found_first = false;
    let mut first_ind = 0;
    for (i, item) in chars {
        if !found_first && item == ' ' { found_first = true; first_ind = i; }
        else if item == ' ' { return &s[first_ind+1..i];}
    }
    return &s[..];
}