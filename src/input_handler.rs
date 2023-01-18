use std::io;
use super::{contributors, QUIT_COMMAND};

pub fn get_user_input(query:&str, buffer:&mut String) -> String {
    println!("{}", query);
    let result = io::stdin().read_line(buffer);

    if result.is_err() {
        panic!("{:?}", result.err());
    }

    buffer.trim().to_owned()
}


pub fn get_store_from_input() -> String {
    let mut store = String::new();
    get_user_input("Which store is the receipt from?", &mut store)
}


pub fn get_paid_by_from_input() -> String {
    let mut paid_by = String::new();
    get_user_input("Who paid for the receipt?", &mut paid_by)
}


pub fn get_contributors_from_input(paid_by:&str) -> contributors::Contributors {
    let mut contributors = String::new();
    let mut contributors = get_user_input("Write each person who contributed to the receipt in a comma separated list, not including yourself (e.g.: John, Jane, Jim)", &mut contributors);
    contributors.push_str(", ");
    contributors.push_str(&paid_by);
    contributors::Contributors::new(&contributors)
}


pub fn get_date_from_input() -> Option<String> {
    let mut date = String::new();
    let date = get_user_input("Enter date for purchase (YYYY-MM-DD). Leave blank for today.", &mut date);
    if !date.is_empty() {
        let ymd: Vec<u32> = date.split('-')
            .map(|d| d.parse().unwrap())
            .collect();
        let receipt_date = chrono::NaiveDate::from_ymd_opt(ymd[0] as i32, ymd[1], ymd[2]);
        Some(receipt_date.unwrap().to_string())
    } else {
        None
    }
}


pub fn get_item_name_from_input(item_number:&u32) -> String {
    let mut input = String::new();
    let msg = format!("Enter name for item {}", &item_number);
    get_user_input(&msg, &mut input)
}


pub fn get_item_price_from_input(item_number:&u32) -> Option<f32> {
    let mut input = String::new();
    let msg = format!("Enter price for item {}", &item_number);
    let price = get_user_input(&msg, &mut input).replace(",", ".");
    
    if price.eq(QUIT_COMMAND) {
        return None;
    }
    Some(price.parse::<f32>().unwrap())
}


pub fn get_discount_from_input(item_number:&u32) -> Option<f32> {
    let mut input = String::new();
    let msg = format!("Enter discount for item {}", &item_number);
    let discount = get_user_input(&msg, &mut input).replace(",", ".");
    
    if discount.eq(QUIT_COMMAND) {
        return None;
    }
    Some(discount.parse().unwrap())
}


pub fn get_contributor_marking_from_input(item_number:&u32, contributors:&contributors::Contributors) -> String {
    let mut input = String::new();

    println!("{}", &contributors.to_string());
    let msg = format!("Mark contributors for item {} with 'x', and non-contributors with 'o'.", &item_number);
    get_user_input(&msg, &mut input)
}


pub fn get_command_from_input() -> String {
    let mut command = String::new();
    get_user_input("What would you like to do? (/quit, /new)", &mut command)
}