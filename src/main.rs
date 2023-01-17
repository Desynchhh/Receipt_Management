extern crate receipt_management;
use receipt_management::{*, contributors::Contributors};
use chrono;

const QUIT_COMMAND: &str = "/quit";
const NEW_RECEIPT_COMMAND: &str = "/new";

fn main() {
    loop {
        let mut command = String::new();
        let command = get_user_input("What would you like to do? (/quit, /new)", &mut command);
        println!("current command is: {}", command);

        match command.as_str() {
            QUIT_COMMAND => {
                break;
            },
            NEW_RECEIPT_COMMAND => {
                new_receipt();
            },
            _ => {
                println!("Unrecognized command. Please try again.");
                continue;
            }
        };
    }
    println!("Thank you for using Desynch's receipt management system! Come again soon! :)");
}

fn new_receipt() {
    let mut paid_by = String::new();
    let paid_by = get_user_input("Who paid for the receipt?", &mut paid_by);
    println!("(paid_by is: {})", paid_by);
    
    let mut store = String::new();
    let store = get_user_input("Which store is the receipt from?", &mut store);
    println!("(store is: {})", store);
    
    let mut contributors = String::new();
    let mut contributors = get_user_input("Write each person who contributed to the receipt in a comma separated list, not including yourself (e.g.: John, Jane, Jim)", &mut contributors);
    contributors.push_str(", ");
    contributors.push_str(&paid_by);
    let contributors = contributors::Contributors::new(&contributors);
    println!("(contributors is: {:?})", contributors);
    
    let mut date = String::new();
    let date = get_user_input("When was the receipt purchased? (Leave blank if today. Else, enter the format YYYY-MM-D)", &mut date);
    let date = if !date.is_empty() {
        let ymd: Vec<u32> = date.split('-')
            .map(|d| d.parse().unwrap())
            .collect();
        chrono::NaiveDate::from_ymd_opt(ymd[0] as i32, ymd[1], ymd[2])
    } else {
        None
    };
    println!("(date is: {:?})", date);

    let mut receipt = receipt::Receipt::new(&paid_by, &store, date, None);

    println!("{:?}", receipt);
    
    let mut item_number = 1;
    loop {
        let mut input = String::new();
        let msg = format!("Enter name for item {}", &item_number);
        let name = get_user_input(&msg, &mut input);


        if input.trim().eq(QUIT_COMMAND) {
            break;
        }
        
        input.clear();
        let msg = format!("Enter price for item {}", &item_number);
        let price = get_user_input(&msg, &mut input);
        
        if input.trim().eq(QUIT_COMMAND) {
            break;
        }
        
        println!("(price is {})", price);
        
        let price = price.parse::<f32>().unwrap();
        
        input.clear();
        let msg = format!("Enter discount for item {}", &item_number);
        let discount = get_user_input(&msg, &mut input);
        
        if input.trim().eq(QUIT_COMMAND) {
            break;
        }
        
        let discount: Option<f32> = if discount.is_empty() {
            None
        } else {
            Some(discount.parse().unwrap())
        };

        input.clear();
        println!("{}", contributors.to_string());
        let msg = format!("Mark contributors for item {} with 'x', and non-contributors with 'o'.", &item_number);
        let contributor_marking = get_user_input(&msg, &mut input);

        let item_contributors = Contributors::from_contributor_marking(&contributor_marking, &contributors);
        let item = item::Item::new(name, price, discount, item_contributors);
        receipt.add_item(item);

        item_number += 1;
    }
    println!("\n\n{:#?}\n", receipt);
    
    for contributor in contributors.names.iter().filter(|c| c.as_str() != paid_by) {
        let owed = receipt.calc_contributor_payment(&contributor);
        println!("{} owes {:.2} to {} for the receipt.", contributor, owed, paid_by);
    }
    println!();
}
