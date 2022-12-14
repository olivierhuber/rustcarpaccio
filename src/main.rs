use std::io;

fn main() {
    println!("Welcome to the GoodSell Hardware-store!");

    println!("Please enter your 2-letter state code?");

    let mut state = String::new();

    io::stdin()
        .read_line(&mut state)
        .expect("Failed to read line");

    let state = state.trim().to_uppercase();

    let tax: f64 = match state.as_str() {
        "UT" => 6.85,
        "NV" => 8.00,
        "TX" => 6.25,
        "AL" => 4.00,
        "CA" => 8.25,
        _ => panic!("Unknown state code {state}")
    };

    println!("Welcome to our {state} store! State tax is {tax:.2}%");

    println!("Please enter the item price?");

    let mut price = String::new();

    io::stdin()
        .read_line(&mut price)
        .expect("Failed to read line");

    let price: f64 = price.trim().parse().expect("Please type a number!");

    println!("Please enter the number of item(s)?");

    let mut items = String::new();

    io::stdin()
        .read_line(&mut items)
        .expect("Failed to read line");

    let items: u32 = items.trim().parse().expect("Please type a number!");

    let total_price: f64 = items as f64 * price;

    let discount = if total_price > 50_000.0 {
        15.0
    } else if total_price > 10_000.0 {
        10.0
    } else if total_price > 7_000.0 {
        7.0
    } else if total_price > 5_000.0 {
        5.0
    } else if total_price > 1_000.0 {
        3.0
    } else {
        0.0
    };

    let discounted_price: f64 = total_price * (1.0 - discount/100.0);

    let total_price = discounted_price * (1.0 + tax/100.0);

    println!("This will cost you ${total_price:.2}");
}
