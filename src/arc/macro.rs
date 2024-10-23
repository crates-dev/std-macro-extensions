/// Arc macro
///
/// # Parameters
/// - `T`: The data to be wrapped in an `Arc`.
///
/// # Returns
/// - An instance of `Arc<T>`.
///
/// # Example
/// ```rust
/// use std_macro_extensions::*;
/// let value = arc!(1);
/// ```
#[macro_export]
macro_rules! arc {
    ($val:expr) => {
        std::sync::Arc::new($val)
    };
}
