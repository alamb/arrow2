mod basic;
mod nested;

pub use basic::array_to_page;
pub(super) use basic::{encode_delta, ord_binary};
pub use nested::array_to_page as nested_array_to_page;
