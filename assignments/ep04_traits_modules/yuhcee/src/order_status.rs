#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}