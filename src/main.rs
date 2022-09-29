use std::io;

fn main() {
    println!("Welcome to the GoodSell Hardware-store!");

    println!("You are buying from our California store!");

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

    const CA_TAX_RATE: f64 = 8.25;
    let total_price: f64 = items as f64 * price * (1.0 + CA_TAX_RATE/100.0);

    println!("This will cost you ${total_price:.2}");
}
