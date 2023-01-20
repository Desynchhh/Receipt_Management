use std::{fs, io};
use serde_json;
use super::receipt::Receipt;

const JSON_PATH:&str = "receipts";


fn ensure_directory_exists(directory:&str) -> Result<(), io::Error> {
    fs::create_dir_all(format!("{}\\{}", JSON_PATH, directory))?;
    Ok(())
}


pub fn receipt_to_json(receipt:&Receipt) {
    let directory_name = &receipt.date;
    match ensure_directory_exists(directory_name) {
        Err(e) => panic!("{}", e),
        Ok(_) => {
            let json_string = serde_json::to_string(receipt);
            match json_string {
                Err(e) => panic!("{}", e),
                Ok(contents) => {
                    let filename = format!("{} - {}.json", receipt.store, receipt.subtotal);
                    let path = format!("{}/{}/{}", JSON_PATH, directory_name, filename);
        
                    fs::write(path, contents)
                        .expect("Something went wrong writing to the json file");
                }
            }
        }
    }
}
