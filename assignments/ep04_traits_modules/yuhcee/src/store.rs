use crate::product::Product;
use crate::order::Order;
use crate::order_status::OrderStatus;

pub struct Store {
    products: Vec<Product>,
    orders: Vec<Order>,
    next_order_id: u32,
    next_product_id: u32,
}

impl Store {
    pub fn new() -> Self {
        Store {
            products: Vec::new(),
            orders: Vec::new(),
            next_order_id: 1,
            next_product_id: 1,
        }
    }

    pub fn add_product(&mut self, name: &str, price: f64, stock: u32) {
        let product = Product {
            id: self.next_product_id,
            name: name.to_string(),
            price,
            stock,
        };
        self.products.push(product);
        self.next_product_id += 1;
        println!("Product added successfully!");
    }

    pub fn place_order(&mut self, customer_name: &str, product_id: u32, quantity: u32) {
        match self.products.iter_mut().find(|p| p.id == product_id) {
            Some(product) => {
                if let Ok(()) = product.reduce_stock(quantity) {
                    let order = Order {
                        id: self.next_order_id,
                        customer_name: customer_name.to_string(),
                        product_id,
                        quantity,
                        status: OrderStatus::Pending,
                    };
                    self.orders.push(order);
                    self.next_order_id += 1;
                    println!("Order placed successfully!");
                } else {
                    println!("Error: Insufficient stock.");
                }
            }
            None => println!("Error: Product not found."),
        }
    }

    pub fn update_order_status(&mut self, order_id: u32, new_status: OrderStatus) {
        if let Some(order) = self.orders.iter_mut().find(|o| o.id == order_id) {
            order.update_status(new_status);
            println!("Order status updated successfully!");
        } else {
            println!("Error: Order not found.");
        }
    }

    pub fn list_products(&self) {
        println!("\n--- Products ---");
        for p in &self.products {
            println!("ID: {}, Name: {}, Price: ${}, Stock: {}", p.id, p.name, p.price, p.stock);
        }
    }

    pub fn list_orders(&self) {
        println!("\n--- Orders ---");
        for o in &self.orders {
            println!(
                "ID: {}, Customer: {}, Product ID: {}, Quantity: {}, Status: {:?}",
                o.id, o.customer_name, o.product_id, o.quantity, o.status
            );
        }
    }
}
