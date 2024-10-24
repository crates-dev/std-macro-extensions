/// Creates a new `Cell` instance.
///
/// This macro takes an expression and wraps it in a `Cell`, providing interior mutability for the given value.
/// The `Cell` type allows for mutation of its contents even when shared among multiple references.
///
/// # Examples
/// ```
/// use std_macro_extensions::*;
/// let cell_value = cell!(5);
/// assert_eq!(cell_value.get(), 5);
/// cell_value.set(10);
/// assert_eq!(cell_value.get(), 10);
/// ```
#[macro_export]
macro_rules! cell {
    ($val:expr) => {
        std::cell::Cell::new($val)
    };
}
