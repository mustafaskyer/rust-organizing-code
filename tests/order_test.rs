use basics::{Category, Customer, Order, Product};
mod helpers;
#[test]
fn test_total_bill_without_discount() {
    let product = Product::new(1, String::from("Book"), 19.9, Category::Books);

    let customer = Customer::new(1, String::from("Name"), String::from("name@domain.com"));

    let order = Order::new(1, product, customer, 3);

    assert_eq!(format!("{:.2}", order.total_bill()), "65.67");
}

#[test]
fn test_total_bill_wit_discount() {
    helpers::common_setup();
    let product = Product::new(1, String::from("Book"), 19.9, Category::Books);

    let customer = Customer::new(1, String::from("Name"), String::from("name@domain.com"));

    let order = Order::new(1, product, customer, 10);

    assert_eq!(format!("{:.2}", order.total_bill()), "197.01");
}
