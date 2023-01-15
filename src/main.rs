use std::io;

const EXIT_PROGRAM_COMMAND: &str = "/quit";
const NEW_RECEIPT_COMMAND: &str = "/new";
const LOOKUP_RECEIPT_COMMAND: &str = "/lookup";

fn main() {
    loop {
        let mut command = String::new();
        let run_command = get_user_input("What would you like to do? (/quit, /new, /lookup)", &mut command);
        if run_command.is_err() {
            break;
        }
        match command.trim().to_lowercase().as_str() {
            EXIT_PROGRAM_COMMAND => break,
            NEW_RECEIPT_COMMAND => {
                new_receipt();
            },
            LOOKUP_RECEIPT_COMMAND => {
                println!("NYI");
            },
            _ => {
                println!("Unrecognized command. Please try again.");
            }
        };
    }
}

fn new_receipt() {
    let mut paid_by = String::new();
    let create_receipt = get_user_input("Who paid for the receipt?", &mut paid_by);
    let mut receipt = receipt_management::Receipt::new(paid_by.trim().to_owned());

    let mut item_number = 1;
    
    if create_receipt.is_ok() {
        loop {
            let (item_name, price, discount, pay_percentage) = match get_item_from_user(&item_number) {
                Ok(item) => item,
                Err(_) => break
            };
            let item = receipt_management::Item::new(item_name, price, discount, pay_percentage);
            receipt.add_item(item);
            item_number += 1;
        }
        receipt.calc_subtotal();
        println!("{:?}", receipt);
    }
}

fn get_user_input(message:&str, buffer:&mut String) -> Result<(), bool> {
    println!("{}", message);
    io::stdin().read_line(buffer).expect("Something went wrong");
    
    if buffer.trim().to_lowercase().eq(EXIT_PROGRAM_COMMAND) {
        return Err(false);
    }
    Ok(())
}

fn get_item_from_user(item_number: &i32) -> Result<(String, f32, f32, u8), bool> {
    let mut item_name = String::new();
    let mut price = String::new();
    let mut discount = String::new();
    let mut pay_percentage = String::new();

    get_user_input(format!("Enter item name for item {}:", item_number).as_str(), &mut item_name)?;
    let item_name = item_name.trim().to_owned();

    get_user_input(format!("Enter item price for item {}:", item_number).as_str(), &mut price)?;
    let price: f32 = price.trim().parse().unwrap();
    
    get_user_input(format!("Enter item discount for item {}:", item_number).as_str(), &mut discount)?;
    let discount: f32 = discount.trim().parse().unwrap();
    
    get_user_input(format!("Enter pay percentage for item {}:", item_number).as_str(), &mut pay_percentage)?;
    let pay_percentage: u8 = pay_percentage.trim().parse().unwrap();

    Ok((item_name, price, discount, pay_percentage))
}
