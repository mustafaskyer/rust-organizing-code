use basics::Category;
use basics::Customer;
use basics::Order;
use basics::Product;

fn main() {
    let product = Product::new(1, String::from("Laptop"), 100.00, Category::Electronice);

    let customer = Customer::new(1, String::from("Name"), String::from("mail@domain.com"));

    let order = Order::new(1, product, customer, 1);

    println!("Tota cost: ${}", order.total_bill())
}
