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
        "CA" => 8.25,
        "TX" => 6.25,
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

    let mut discount: f64 = 0.0;

    if total_price > 50_000.0 {
        discount = 15.0;
    }

    let discounted_price: f64 = total_price * (1.0 - discount/100.0);

    let total_price = discounted_price * (1.0 + tax/100.0);

    println!("This will cost you ${total_price:.2}");
}
