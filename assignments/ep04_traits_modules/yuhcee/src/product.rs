pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub stock: u32,
}

impl Product {
    pub fn is_in_stock(&self, quantity: u32) -> bool {
        self.stock >= quantity
    }

    pub fn reduce_stock(&mut self, quantity: u32) -> Result<(), String> {
        if self.is_in_stock(quantity) {
            self.stock -= quantity;
            Ok(())
        } else {
            Err("Insufficient stock.".to_string())
        }
    }
}