use super::contributors::Contributors;

#[derive(Debug, PartialEq)]
pub struct Item {
    name: String,
    pub price: f32,
    pub discount: Option<f32>,
    pub contributors: Contributors,
}

impl Item {
    pub fn new(name: String, price: f32, discount:Option<f32>, contributors: Contributors) -> Item {
        Item {
            name,
            discount,
            price,
            contributors: contributors,
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