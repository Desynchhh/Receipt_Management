use super::item::Item;
use chrono::{ NaiveDate };

#[derive(Debug)]
pub struct Receipt {
    store: String,
    date: NaiveDate,
    paid_by: String,
    items: Vec<Item>,
}

impl Receipt {
    pub fn new(
        paid_by:&str,
        store:&str,
        date:Option<NaiveDate>,
        items:Option<Vec<Item>>
    ) -> Receipt {
        let today = chrono::Local::now().naive_local().date();

        let mut receipt = Receipt { 
            paid_by: paid_by.to_owned(),
            store: store.to_owned(),
            date:today,
            items:vec![]
        };

        if items.is_some() {
            receipt.add_items(items.unwrap());
        }

        if date.is_some() {
            receipt.set_date(date.unwrap());
        }

        receipt
    }


    pub fn set_date(&mut self, date:NaiveDate) {
        self.date = date;
    }


    pub fn add_items(&mut self, items: Vec<Item>) {
        self.items.extend(items);
    }


    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }


    fn calc_contributor_payment(&self, contributor:&str) -> f32 {
        let items: Vec<&Item> = self.items
            .iter()
            .filter(|item| item.contributors.names.contains(&contributor.to_owned()))
            .collect();

        let mut owed = 0.0;
        for item in items {
            owed += match item.discount {
                None => item.price / item.contributors.names.len() as f32,
                Some(d) => (item.price - d) / item.contributors.names.len() as f32
            }
        }
        owed
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::contributors::Contributors;

    #[test]
    fn contributor_payment_without_discount() {
        let mut receipt = Receipt::new("Mikkel", "Super Brugsen", None, None);
        receipt.add_items(vec![
            Item::new(
                String::from("Milk"),
                10.0,
                None,
                Contributors::new("Mikkel, Thea"),
            ),
            Item::new(
                String::from("White chocolate"),
                10.0,
                None,
                Contributors::new("Thea"),
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
                Contributors::new("Mikkel, Thea"),
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                Contributors::new("Thea"),
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
                Contributors::new("Mikkel, Thea"),
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                Contributors::new("Thea"),
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
                Contributors::new("Mikkel, Thea"),
            ),
            Item::new(
                String::from("White chocolate"),
                20.0,
                Some(5.0),
                Contributors::new("Thea"),
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
                Contributors::new("Mikkel, Thea"),
            ),
            Item::new(
                String::from("White chocolate"),
                19.95,
                None,
                Contributors::new("Mikkel, Thea"),
            ),
        ]);

        assert_eq!(
            receipt.items,
            vec![
                Item::new(
                    String::from("Milk"),
                    12.95,
                    None,
                    Contributors::new("Mikkel, Thea")
                ),
                Item::new(
                    String::from("White chocolate"),
                    19.95,
                    None,
                    Contributors::new("Mikkel, Thea")
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
            Contributors::new("Mikkel, Thea"),
        ));

        assert_eq!(
            receipt.items,
            vec![Item::new(
                String::from("Mikkel"),
                12.95,
                None,
                Contributors::new("Mikkel, Thea")
            )]
        )
    }
}
