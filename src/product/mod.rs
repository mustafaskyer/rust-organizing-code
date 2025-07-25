pub use category::Category;
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: Category,
}

mod category;

impl Product {
    fn claculate_tax(&self) -> f64 {
        self.price * 0.1
    }

    pub fn new(id: u64, name: String, price: f64, category: Category) -> Product {
        Product {
            id,
            name,
            price,
            category,
        }
    }

    pub fn product_price(&self) -> f64 {
        self.price + self.claculate_tax()
    }
}
