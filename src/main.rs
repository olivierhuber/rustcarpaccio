use std::io;

fn main() {
    println!("Welcome to the GoodSell Hardware-store!");

    println!("Please input your email to receive our newsletter:");

    let mut email = String::new();

    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");

    println!("Thank you for subscribing to our newsletter {email}");
}
