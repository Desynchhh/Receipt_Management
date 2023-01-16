use super::item::Item;

#[derive(Debug)]
pub struct Receipt {
    paid_by: String,
    items: Vec<Item>,
}

impl Receipt {
    pub fn new(paid_by: String) -> Receipt {
        Receipt {
            paid_by,
            items: vec![],
        }
    }

    pub fn add_items(&mut self, items: Vec<Item>) {
        self.items.extend(items);
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn calc_contributor_payment(&self, contributor:&String) -> f32 {
        let items: Vec<&Item> = self.items
            .iter()
            .filter(|item| item.contributors.contains(contributor))
            .collect();

        let mut owed = 0.0;
        for item in items {
            owed += match item.discount {
                None => item.price / item.contributors.len() as f32,
                Some(d) => (item.price - d) / item.contributors.len() as f32
            }
        }
        owed
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contributor_payment_without_discount() {
        let mut receipt = Receipt::new(String::from("Mikkel"));
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                10.0,
                None,
                vec![String::from("Mikkel"), String::from("Thea")],
            ),
            Item::new(
                String::from("White chocolate"),
                10.0,
                None,
                vec![String::from("Thea")],
            ),
        ]);

        let expected_result = 15.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn contributor_payment_with_discount() {
        let mut receipt = Receipt::new(String::from("Mikkel"));
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.0,
                Some(2.0),
                vec![String::from("Mikkel"), String::from("Thea")],
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                vec![String::from("Thea")],
            ),
        ]);

        let expected_result = 20.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn contributor_payment_mixed_discount() {
        let mut receipt = Receipt::new(String::from("Mikkel"));
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.0,
                None,
                vec![String::from("Mikkel"), String::from("Thea")],
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                vec![String::from("Thea")],
            ),
        ]);

        let expected_result = 21.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn contributor_payment_discount_zero() {
        let mut receipt = Receipt::new(String::from("Mikkel"));
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.0,
                Some(0.0),
                vec![String::from("Mikkel"), String::from("Thea")],
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                vec![String::from("Thea")],
            ),
        ]);

        let expected_result = 21.0;
        let actual_result: f32 = receipt.calc_contributor_payment(&String::from("Thea"));

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn add_multiple_items() {
        let mut receipt = Receipt::new(String::from("Mikkel"));
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                12.95,
                None,
                vec![String::from("Mikkel"), String::from("Thea")],
            ),
            Item::new(
                String::from("White chocolate"),
                19.95,
                None,
                vec![String::from("Thea")],
            ),
        ]);

        assert_eq!(
            receipt.items,
            vec![
                Item::new(
                    String::from("Milk"),
                    12.95,
                    None,
                    vec![String::from("Mikkel"), String::from("Thea")]
                ),
                Item::new(
                    String::from("White chocolate"),
                    19.95,
                    None,
                    vec![String::from("Thea")]
                )
            ]
        );
    }

    #[test]
    fn add_single_item() {
        let mut receipt = Receipt::new(String::from("Mikkel"));
        receipt.add_item(Item::new(
            String::from("Mikkel"),
            12.95,
            None,
            vec![String::from("Mikkel"), String::from("Thea")],
        ));

        assert_eq!(
            receipt.items,
            vec![Item::new(
                String::from("Mikkel"),
                12.95,
                None,
                vec![String::from("Mikkel"), String::from("Thea")]
            )]
        )
    }
}
