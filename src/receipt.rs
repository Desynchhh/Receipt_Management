use super::item::Item;
use chrono;
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    pub store: String,
    pub date: String,
    pub paid_by: String,
    pub items: Vec<Item>,
    pub subtotal: f32,
    pub contributor_to_pay: HashMap<String, f32>
}

impl Receipt {
    pub fn new(
        paid_by:&str,
        store:&str,
        date:Option<String>,
        items:Option<Vec<Item>>
    ) -> Receipt {
        let today = chrono::Local::now().naive_local().date().to_string();

        let mut receipt = Receipt { 
            paid_by: paid_by.to_owned(),
            store: store.to_owned(),
            date:today,
            items:vec![],
            subtotal: 0.0,
            contributor_to_pay: HashMap::new(),
        };

        if items.is_some() {
            receipt.add_items(items.unwrap());
        }

        if date.is_some() {
            receipt.set_date(date.unwrap());
        }

        receipt
    }


    pub fn set_date(&mut self, date:String) {
        self.date = date;
    }


    pub fn add_items(&mut self, items: Vec<Item>) {
        self.items.extend(items);
    }


    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn calc_subtotal(&mut self) {
        let mut total = 0.0;
        for item in &self.items {
            total += item.calc_paid();
        }
        self.subtotal = total;
    }

    pub fn calc_contributor_payment(&mut self, contributor:&str) -> f32 {
        let items = self.items
            .iter()
            .filter(|item| item.contributors.contains(&contributor.to_owned()));

        let mut owed = 0.0;
        for item in items {
            owed += match item.discount {
                None => item.price / item.contributors.len() as f32,
                Some(d) => (item.price - d) / item.contributors.len() as f32
            }
        }

        self.contributor_to_pay.insert(contributor.to_string(), owed);
        owed
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contributor_payment_without_discount() {
        let mut receipt = Receipt::new("Mikkel", "Super Brugsen", None, None);
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                10.0,
                None,
                vec!["Mikkel".to_string(), "Thea".to_string()],
            ),
            Item::new(
                String::from("White chocolate"),
                10.0,
                None,
                vec!["Thea".to_string()],
            ),
            Item::new(
                String::from("Chips"),
                20.0,
                None,
                vec!["Mikkel".to_string()],
            ),
        ]);

        let expected_result = 15.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn contributor_payment_with_discount() {
        let mut receipt = Receipt::new("Mikkel", "Super Brugsen", None, None);
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.0,
                Some(2.0),
                vec!["Mikkel".to_string(), "Thea".to_string()],
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                vec!["Thea".to_string()],
            ),
        ]);

        let expected_result = 20.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn contributor_payment_mixed_discount() {
        let mut receipt = Receipt::new("Mikkel", "Super Brugsen", None, None);
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.0,
                None,
                vec!["Mikkel".to_string(), "Thea".to_string()],
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                vec!["Thea".to_string()],
            ),
        ]);

        let expected_result = 21.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn contributor_payment_discount_zero() {
        let mut receipt = Receipt::new("Mikkel", "Super Brugsen", None, None);

        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.0,
                Some(0.0),
                vec!["Mikkel".to_string(), "Thea".to_string()],
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                vec!["Thea".to_string()],
            ),
        ]);

        let expected_result = 21.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn add_multiple_items() {
        let mut receipt = Receipt::new("Mikkel", "Super Brugsen", None, None);

        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.95,
                None,
                vec!["Mikkel".to_string(), "Thea".to_string()],
            ),
            Item::new(
                String::from("White chocolate"),
                19.95,
                None,
                vec!["Mikkel".to_string(), "Thea".to_string()],
            ),
        ]);

        assert_eq!(
            receipt.items,
            vec![
                Item::new(
                    String::from("Milk"),
                    12.95,
                    None,
                    vec!["Mikkel".to_string(), "Thea".to_string()]
                ),
                Item::new(
                    String::from("White chocolate"),
                    19.95,
                    None,
                    vec!["Mikkel".to_string(), "Thea".to_string()]
                )
            ]
        );
    }

    #[test]
    fn add_single_item() {
        let mut receipt = Receipt::new("Mikkel", "Super Brugsen", None, None);

        receipt.add_item(Item::new(
            String::from("Mikkel"),
            12.95,
            None,
            vec!["Mikkel".to_string(), "Thea".to_string()],
        ));

        assert_eq!(
            receipt.items,
            vec![Item::new(
                String::from("Mikkel"),
                12.95,
                None,
                vec!["Mikkel".to_string(), "Thea".to_string()]
            )]
        )
    }
}
