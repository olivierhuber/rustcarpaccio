use std::io;

fn main() {
    println!("Welcome to the GoodSell Hardware-store!");

    let tax = ask_for_state_and_get_tax();
    let price = ask_for_item_price();
    let items = ask_for_number_of_items();

    let total_price = items * price;
    let discount = get_discount(total_price);
    let discounted_price = total_price * (1.0 - discount/100.0);
    let total_price = discounted_price * (1.0 + tax/100.0);

    println!("This will cost you ${total_price:.2}");
}

enum State {
    Utah,
    Nevada,
    Texas,
    Alabama,
    California
}

impl From<&str> for State {
    fn from(state: &str) -> Self {
        match state {
            "UT" => State::Utah,
            "NV" => State::Nevada,
            "TX" => State::Texas,
            "AL" => State::Alabama,
            "CA" => State::California,
            _ => panic!("Unknown state code {state}")
        }
    }
}

fn ask_for_state_and_get_tax() -> f64 {
    println!("Please enter your 2-letter state code?");

    let mut state = String::new();

    io::stdin()
        .read_line(&mut state)
        .expect("Failed to read line");

    let state = state.trim().to_uppercase().as_str().into();

    let tax = match state {
        State::Utah => 6.85,
        State::Nevada => 8.00,
        State::Texas => 6.25,
        State::Alabama => 4.00,
        State::California => 8.25,
    };

    println!("Welcome to our {state} store! State tax is {tax:.2}%");

    tax
}

fn get_discount(total_price: f64) -> f64 {
    if total_price > 50_000.0 {
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
    }
}

fn ask_for_item_price() -> f64 {
    ask_for_number_input("Please enter the item price?")
}

fn ask_for_number_of_items() -> u32 {
    ask_for_number_input("Please enter the number of item(s)?")
}

fn ask_for_number_input(question: &str) -> f64 {
    println!(question);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().parse().expect("Please type a number!")
}