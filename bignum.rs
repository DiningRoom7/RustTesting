fn main(){
    println!("Biggest u128: {}", std::u128::MAX);
    println!("Biggest f32:  {:+e}", std::f64::MAX);
    let fact52: f64 = factorial(52);
    println!("52 Factorial: {}", fact52);
}

fn factorial(mut num: u32) -> f64{
    let mut out: f64 = num as f64;
    while num > 1 {
        out *= (num-1) as f64;
        num -= 1;
    }
    return out;
}