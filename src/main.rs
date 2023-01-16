use receipt_management::*;

fn main() {
    let mut name = String::new();

    let name = get_user_input("What is your name?", &mut name);

    println!("Your name is {}!", name);
}
