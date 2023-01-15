#[derive(Debug)]
pub struct Item {
    name: String,
    price: f32,
    discount: f32,
    pay_percentage: u8
}

impl Item {
    pub fn new(name: String, price: f32, discount: f32, pay_percentage: u8) -> Item {
        Item {
            name,
            price,
            discount,
            pay_percentage
        }
    }
}

#[derive(Debug)]
pub struct Receipt {
    paid_by: String,
    items: Vec<Item>,
    subtotal: f32
}

impl Receipt {
    pub fn new(paid_by: String) -> Receipt {
        Receipt { paid_by, items: vec![], subtotal: 0.0 }
    }

    pub fn add_item(&mut self, item:Item) {
        self.items.push(item);
    }

    pub fn set_owner(&mut self, name:String) {
        self.paid_by = name;
    }

    pub fn calc_subtotal(&mut self) {
        for item in &self.items {
            self.subtotal += item.price;
        }
    }
}