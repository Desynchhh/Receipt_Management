#[derive(Debug, PartialEq)]
pub struct Item {
    name: String,
    pub price: f32,
    pub discount: Option<f32>,
    pub contributors: Vec<String>,
}

impl Item {
    pub fn new(name: String, price: f32, discount:Option<f32>, contributors: Vec<String>) -> Item {
        Item {
            name,
            discount,
            price,
            contributors,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_item() {
        let manuel_item = Item {
            name: String::from("Milk"),
            price: 12.95,
            discount: None,
            contributors: vec![String::from("Mikkel"), String::from("Thea")],
        };

        let constructor_item = Item::new(
            String::from("Milk"),
            12.95,
            None,
            vec![String::from("Mikkel"), String::from("Thea")]
        );

        assert_eq!(manuel_item, constructor_item);
    }
}