// pub mod <module file name>
pub mod hello;
pub mod hello2;
// pub use crate::<module name>::<module file name>::<function name>
pub use crate::module::hello::say_hello;
pub use crate::module::hello2::say_hello2;
