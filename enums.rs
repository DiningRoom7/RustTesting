enum IpAddrKind {
    V4,
    V6,
}
//IpAddrKind::v4 and IpAddrKind::v6 are of the same type
//So we can make a function that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {
    //Match can but used to deduce the specifics of an enum
    println!("{}",match ip_kind {
        IpAddrKind::V4 => "IPV4 Address",
        IpAddrKind::V6 => "IPV6 Address",
    });
}

//You can also put data directly into an enum variant
enum IpAddrNew {
    V4(String),
    V6(String),
}
//And again we can make a function that takes the enum type
fn foo(ip: IpAddrNew) {
    //match can be used like before
    println!("{}", match ip {
        IpAddrNew::V4(value) => "IPV4 Address: ".to_owned() + &value,
        IpAddrNew::V6(value) => "IPV6 Address: ".to_owned() + &value,
    })

    //Or "if let" can be used
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddrNew::V4(String::from("127.0.0.1"));
    let loopback = IpAddrNew::V6(String::from("::1"));
    foo(home);
    foo(loopback);
}