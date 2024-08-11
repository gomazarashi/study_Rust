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

fn value_in_cent(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(UsState::Alaska);

    println!("Penny's value in cents: {}", value_in_cent(coin1));
    println!("Nickel's value in cents: {}", value_in_cent(coin2));
    println!("Dime's value in cents: {}", value_in_cent(coin3));
    println!("Quarter's value in cents: {}", value_in_cent(coin4));
}
