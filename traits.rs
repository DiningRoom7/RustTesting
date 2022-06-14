//Defining a trait Summary with one method summarize
trait Summary {
    fn summarize(&self) -> &str;
    //Structs that implement traits must implement all the methods in a trait
    //If another method was defined here I must also implement it in the impl Summary for Article
}

//Some traits such as Debug, Ord, and Clone can be automatically implemented using
#[derive(Debug)]
struct Article {
    content: String
}

//Implementing the methods of the Summary trait for our Article struct
impl Summary for Article {
    fn summarize(&self) -> &str {
        for (i, c) in self.content.chars().enumerate() {
            if c == ' ' { return &self.content[..i]}
        }
        &self.content[..]
    }
}



fn main() {
    let a = Article { content: String::from("Hello there.")};

    //print the summary
    println!("{}", a.summarize());

    //print the debug output
    println!("{:?}", a);
}