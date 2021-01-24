#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny.");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}



fn main() {
    let m = Coin::Quarter(UsState::Alaska);
    value_in_cents(m);

    let five = Some(5);
    let six = plus_one(five);

    println!("five plus one is {:?}", six);

    let mut count = 0;
    // let coin = Coin::Quarter(UsState::Alabama);
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    println!("Count is {}", count);

}
