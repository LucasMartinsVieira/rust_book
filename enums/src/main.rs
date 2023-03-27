// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
//
// #[allow(unused)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     #[allow(unused)]
//     fn call(&self) {}
// }
//
// #[allow(unused_variables)]
// fn main() {
//     let home = IpAddrKind::V4(127, 0, 0, 1);
//     let loopback = IpAddrKind::V6(String::from("::1"));
//     let some_number = Some(5);
//     let some_char = Some('e');
//     let absent_number: Option<i32> = None;
//
//     // let home = IpAddr {
//     //     kind: IpAddrKind::V4,
//     //     address: String::from("127.0.0.1"),
//     // };
//     //
//     // let loopback = IpAddr {
//     //     kind: IpAddrKind::V6,
//     //     address: String::from("::1"),
//     // };
// }

#[allow(unused)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(unused_variables)]
fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);

    let dice_roll = 3;

    match dice_roll {
        3 => println!("Add fancy hat!"),
        7 => println!("Remove fancy hat!"),
        _ => println!("Reroll"),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn if_let() {
    let config_max = Some(3u8);

    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    
    // Same thing, less boilerplate code
    if let Some(max) = config_max  {
        println!("The maximum is configured to be {}", max);
    }
}
