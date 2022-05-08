fn main() {
    let now = std::time::Instant::now();

    for i in 1..=10000000 {
        if i % 1000000 == 0 { println!("{}", i); }
        work(i);
    }

    let later = now.elapsed();

    println!("Time elapsed: {}ms", later.as_millis());
}

fn work(a: i32) -> (i128, i128, i128){
    let mut x: i128 = (a*3/226*4%123*226) as i128;
    let mut y: i128 = 1234243;
    let mut z: i128 = 2341324;
    let xyz: i128 = x*y*z;
    x = xyz/3;
    y = xyz*2;
    z = xyz/12345;
    return (x, y, z);
}