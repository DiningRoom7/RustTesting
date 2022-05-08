fn main() {
    let start = std::time::Instant::now();
    let mut threads = vec![];

    for i in 1..=100 {
        threads.push(std::thread::spawn(move || {
            let res = hailstone(i*22232313222+1);
            print!("Hello from thread {}", i);
            println!(", {} made it to 1 in {} iterations",i*22232313222+1,res);
        })); 
    }

    for i in threads {
        let _ = i.join();
    }

    println!("Time elapsed: {}ms", start.elapsed().as_millis());
}

fn hailstone(mut num: i64) -> i32 {
    let mut iters: i32 = 0;
    while num > 1 {
        if num % 2 == 0 {num /= 2;}
        else {num = num*3+1}
        iters += 1;
    }
    return iters;
}