fn main() {
    //Note that individual fields of the struct cannot be mutable
    //The entire struct must either be mutable or not mutable
    let mut m = Message{
        content: String::from("Hello, World!"),
        to: String::from("World"),
        from: String::from("Computer"),
        sent: false,
    };
    Message::print(&m);

    println!("SENDING MESSAGE");

    m.send();

    Message::print(&m);

}

struct Message {
    content: String,
    to: String,
    from: String,
    sent: bool,
}
impl Message {
    //Methods always have self as the first argument
    //Methods can take ownership, or borrow mutably or immutably
    fn send(&mut self) {
        //  ^^^^^^^^^ <-- the same as (self: &mut Self)
        //Within an impl block, the type Self is an alias for the type that the impl block is for
        self.sent = true;
    }

    //Associated functions can also appear within impl blocks
    //Naturally this would be called by Message::print(...)
    fn print(m: &Message) {
        println!("{:=<50}", "=");
        println!("From: {}\nTo: {}\nMessage:\n\t{:30}\n\nMessage has {}been sent.",
                m.from,
                m.to,
                m.content, 
                if m.sent {""} else {"not "});
        println!("{:=<50}", "=");
    }
}