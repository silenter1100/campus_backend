pub mod controller;
pub mod entity;
pub mod service;
pub mod routes;

// 重新导出主要类型
pub use entity::*;
pub use service::*;
pub use routes::create_routes;