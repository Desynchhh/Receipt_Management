mod item;
mod receipt;

use item::Item;
use receipt::Receipt;

fn create_receipt(paid_by: &str, items:Option<Vec<Item>>) -> Receipt {
    match items {
        None => Receipt::new(paid_by.to_owned()),
        Some(i) => {
            let mut receipt = Receipt::new(paid_by.to_owned());
            receipt.add_items(i);
            receipt
        }
    }
}

fn create_item(name:&str, price:f32, discount:Option<f32>, contributors:Vec<String>) -> Item {
    Item::new(name.to_owned(), price, discount, contributors)
}