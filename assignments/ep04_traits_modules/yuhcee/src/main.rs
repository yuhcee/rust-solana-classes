use std::io::{self, Write};
use yuhcee_ep04_traits_modules::{
    store::Store,
    order_status::OrderStatus,
};

fn main() {
    let mut store = Store::new();

    store.add_product("MacBook Pro", 1999.99, 8);
    store.add_product("AirPods", 199.99, 3);
    store.add_product("iPhone 14", 899.99, 0);

    loop {
        println!("\nWelcome to the Store! What would you like to do?");
        println!("1. List Products");
        println!("2. Place Order");
        println!("3. Update Order Status");
        println!("4. List Orders");
        println!("5. Exit");

        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => store.list_products(),
            "2" => {
                let mut customer_name = String::new();
                let mut product_id_str = String::new();
                let mut quantity_str = String::new();

                print!("Enter your name: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut customer_name).expect("Failed to read input");

                print!("Enter product ID: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut product_id_str).expect("Failed to read input");

                print!("Enter quantity: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut quantity_str).expect("Failed to read input");

                let product_id = product_id_str.trim().parse().unwrap_or(0);
                let quantity = quantity_str.trim().parse().unwrap_or(0);

                store.place_order(customer_name.trim(), product_id, quantity);
            }
            "3" => {
                let mut order_id_str = String::new();
                let mut status_str = String::new();

                print!("Enter order ID: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut order_id_str).unwrap();

                print!("Enter new status (Pending, Shipped, Delivered, Cancelled): ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut status_str).unwrap();

                let order_id = order_id_str.trim().parse().unwrap_or(0);
                let new_status = match status_str.trim().to_lowercase().as_str() {
                    "pending" => Some(OrderStatus::Pending),
                    "shipped" => Some(OrderStatus::Shipped),
                    "delivered" => Some(OrderStatus::Delivered),
                    "cancelled" => Some(OrderStatus::Cancelled),
                    _ => None,
                };

                if let Some(status) = new_status {
                    store.update_order_status(order_id, status);
                } else {
                    println!("Invalid choice. Please try again.");
                }
            }
            "4" => store.list_orders(),
            "5" => {
                println!("Thank you for using the store!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
