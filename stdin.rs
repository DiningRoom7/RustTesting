//For some reason we need this to use stdout flush function
use std::io::Write;

fn main() {
    let mut guess = String::new();

    print!("Input a number: ");
    //Flush stdout so the input prompt actually gets printed
    let _ = std::io::stdout().flush();

    std::io::stdin().read_line(&mut guess).expect("Failed to read line!");
    
    println!("\nYour number is: {}", guess);
}