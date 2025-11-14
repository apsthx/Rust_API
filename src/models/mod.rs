// Models module - Data Access Layer
// Equivalent to Go's models/ directory

pub mod user;
pub mod order;
pub mod customer;
pub mod product;
pub mod category;
pub mod shop;

// Re-export commonly used models
pub use user::{User, UserModel};
pub use order::{Order, OrderModel};
pub use customer::{Customer, CustomerModel};
pub use product::{Product, ProductModel};
pub use category::{Category, CategoryModel};
pub use shop::{Shop, ShopModel};
