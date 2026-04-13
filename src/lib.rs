mod magic;
pub mod mime;
mod tree;

pub use mime::{detect_from_reader, detect, set_rate_limit};
