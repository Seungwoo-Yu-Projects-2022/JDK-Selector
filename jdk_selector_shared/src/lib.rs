pub mod models;
pub mod utils;

#[macro_export]
macro_rules! print_on_debug {
    ($($rest:tt)*) => {
        #[cfg(debug_assertions)]
        std::dbg!($($rest)*)
    }
}