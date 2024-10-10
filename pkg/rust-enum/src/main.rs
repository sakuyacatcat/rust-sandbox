#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    // enum
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    route(home);
    route(loopback);

    // enum with method
    let q = Message::Quit;
    q.call();
    let m = Message::Move { x: 1, y: 2 };
    m.call();
    let w = Message::Write(String::from("hello"));
    w.call();
    let c = Message::ChangeColor(1, 2, 3);
    c.call();

    // enum and match
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));
}

fn route(ip: IpAddr) {
    println!("{:#?}", ip);
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
