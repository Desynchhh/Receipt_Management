use std::io;

pub mod item;
pub mod receipt;
pub mod contributors;

pub fn get_user_input(query:&str, buffer:&mut String) -> String {
    println!("{}", query);
    let result = io::stdin().read_line(buffer);

    if result.is_err() {
        panic!("{:?}", result.err());
    }

    buffer.trim().to_owned()
}
