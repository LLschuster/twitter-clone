pub mod register;

pub use register::Register;

pub use route::*;

pub mod route {
    pub const ACCCOUNT_REGISTER: &str = "/account/register";
}