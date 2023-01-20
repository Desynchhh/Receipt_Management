use receipt_management::*;

fn main() {
    loop {
        let command = receipt_management::get_command_from_input();

        match command.as_str() {
            QUIT_COMMAND => {
                break;
            },
            NEW_RECEIPT_COMMAND => {
                receipt_management::new_receipt();
            },
            _ => {
                println!("Unrecognized command. Please try again.");
                continue;
            }
        };
    }
    println!("Thank you for using Desynch's receipt management system! Come again soon! :)");
}
