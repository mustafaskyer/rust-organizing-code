use crate::customer::Customer;
use crate::product::Product;
pub struct Order {
    id: u64,
    product: Product,
    customer: Customer,
    quantity: u32,
}

impl Order {
    pub fn calculate_discount(&self) -> f64 {
        if self.quantity > 5 { 0.1 } else { 0.0 }
    }
    pub fn total_bill(&self) -> f64 {
        let discount = self.calculate_discount();
        let total_before_discount: f64 = self.product.product_price() * self.quantity as f64;
        total_before_discount - (total_before_discount * discount)
    }

    pub fn new(id: u64, product: Product, customer: Customer, quantity: u32) -> Order {
        Order {
            id,
            product,
            customer,
            quantity,
        }
    }
}
