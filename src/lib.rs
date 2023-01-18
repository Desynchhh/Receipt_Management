pub mod item;
pub mod receipt;
pub mod contributors;
mod file_manager;
mod input_handler;

pub const QUIT_COMMAND: &str = "/quit";
pub const NEW_RECEIPT_COMMAND: &str = "/new";

pub fn get_command_from_input() -> String {
    input_handler::get_command_from_input()
}


fn new_item(item_number:&u32, contributors:&contributors::Contributors) -> Option<item::Item> {
    let name = input_handler::get_item_name_from_input(&item_number);
    if name.eq(QUIT_COMMAND) {
        return None;
    }

    let price_option = input_handler::get_item_price_from_input(&item_number);
    let price: f32;
    if price_option.is_none() {
        return None;
    } else {
        price = price_option.unwrap();
    }
    
    let discount = input_handler::get_discount_from_input(&item_number);
    if discount.is_none() {
        return None;
    }

    let contributor_marking = input_handler::get_contributor_marking_from_input(&item_number, &contributors);
    if contributor_marking.eq(QUIT_COMMAND) {
        return None;
    }

    let item_contributors = contributors::Contributors::from_contributor_marking(&contributor_marking, &contributors);
    Some(item::Item::new(name, price, discount, item_contributors))
}


pub fn new_receipt() {
    let store = input_handler::get_store_from_input();
    let paid_by = input_handler::get_paid_by_from_input();
    let contributors = input_handler::get_contributors_from_input(&paid_by);
    let date = input_handler::get_date_from_input();

    let mut receipt = receipt::Receipt::new(&paid_by, &store, date, None);
    println!("{:?}", receipt);
    
    let mut item_number:u32 = 1;
    loop {
        let item = new_item(&item_number, &contributors);
        if item.is_none() {
            break;
        }
        receipt.add_item(item.unwrap());
        item_number += 1;
    }
    println!("\n\n{:#?}\n", receipt);
    file_manager::receipt_to_json(&receipt);
    
    for contributor in contributors.names.iter().filter(|c| c.as_str() != paid_by) {
        let owed = receipt.calc_contributor_payment(&contributor);
        println!("{} owes {:.2} to {} for the receipt.", contributor, owed, paid_by);
    }
    println!();
}
