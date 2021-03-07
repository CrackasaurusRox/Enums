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

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
           println!("Found None, Returned None");
           None
        },
        Some(i) => {
            println!("Found {0}, returned {0} + 1", i);
            Some(i + 1)
        },
    }
}