use crate::order_status::OrderStatus;

#[derive(Debug)]
pub struct Order {
    pub id: u32,
    pub customer_name: String,
    pub product_id: u32,
    pub quantity: u32,
    pub status: OrderStatus,
}

impl Order {
    pub fn update_status(&mut self, new_status: OrderStatus) {
        self.status = new_status;
    }

    pub fn print_status(&self) {
        println!("Order #{} status: {:?}", self.id, self.status);
    }
}
