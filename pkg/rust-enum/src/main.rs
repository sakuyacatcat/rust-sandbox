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

    // Option<T> and match
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:#?}", six);
    println!("{:#?}", none);

    // if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
