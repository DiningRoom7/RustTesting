
fn main(){
    let my_vec = vec!(1, 2, 3, 4, 6, 5);

    match IntoIterator::into_iter(my_vec) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            let () = { println!("{}", x); };
        },
    };
}