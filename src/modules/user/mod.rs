pub mod controller;
pub mod entity;
pub mod service;

// 重新导出公共接口
pub use controller::{
    login_handler,
    register_handler,
    get_user_info_handler,
    update_profile_handler,
    logout_handler,
    change_password_handler,
    router,
    LoginRequest,
    RegisterRequest,
    ChangePasswordRequest,
};
pub use entity::{User, UpdateUserProfile};
pub use service::UserService;
