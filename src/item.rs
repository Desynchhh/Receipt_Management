use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
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

    pub fn calc_paid(&self) -> f32 {
        match self.discount {
            None => self.price,
            Some(discount) => self.price - discount
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn create_new_item() {
//         let manuel_item = Item {
//             name: String::from("Milk"),
//             price: 12.95,
//             discount: None,
//             contributors: Contributors::new("Mikkel, Thea"),
//         };

//         let constructor_item = Item::new(
//             String::from("Milk"),
//             12.95,
//             None,
//             Contributors::new("Mikkel, Thea")
//         );

//         assert_eq!(manuel_item, constructor_item);
//     }
// }