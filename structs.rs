fn print_message(m: &Message) {
    println!("{:=<50}", "=");
    println!("From: {}\nTo: {}\nMessage:\n\t{:30}\n\nMessage has {} been sent.",
                                    m.from,
                                    m.to,
                                    m.content, 
                                    if m.sent {""} else {"not"});
    println!("{:=<50}", "=");
}

fn main() {
    //Note that individual fields of the struct cannot be mutable
    //The entire struct must either be mutable or not mutable
    let m = make_message(String::from("Everyone"),
                         String::from("Computer"),
                         String::from("Hello, World!"));
    print_message(&m);

    //Create new message using old one
    let n = Message {
        content: String::from("This is pretty cool!"),
        ..m
    };
    print_message(&n);

    

}

struct Message {
    content: String,
    to: String,
    from: String,
    sent: bool,
}

fn make_message(to: String, from: String, content: String) -> Message {
    Message {
        content, //<--
        to,      //<-- This uses field init shorthand
        from,    //<--
        sent: false, //<-- This one is regular field init
    }
}