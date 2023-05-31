#[cfg_attr(feature = "a", path = "impl_a.rs")]
#[cfg_attr(feature = "b", path = "impl_b.rs")]
#[cfg_attr(not(any(
    feature = "a",
    feature = "b",
)),
path = "stub.rs")]
mod inner;

pub use inner::*;
