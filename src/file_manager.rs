use std::fs;
use serde_json;

use super::receipt::Receipt;

const JSON_PATH:&str = "json/test.json";

pub fn receipt_to_json(receipt:&Receipt) {
    let json_string = serde_json::to_string(receipt);
    match json_string {
        Err(e) => panic!("{}", e),
        Ok(contents) => {
            fs::write(JSON_PATH, contents)
                .expect("Something went wrong writing to the json file");
        }
    }
}