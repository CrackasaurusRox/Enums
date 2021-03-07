enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let p: Coin = Coin::Penny;
    let n: Coin = Coin::Nickel;
    let d: Coin = Coin::Dime;
    let q: Coin = Coin::Quarter;

    println!("A penny is worth {} cents.", value_in_cents(p));
    println!("A nickel is worth {} cents.", value_in_cents(n));
    println!("A dime is worth {} cents.", value_in_cents(d));
    println!("A quarter is worth {} cents.", value_in_cents(q));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}