/// Creates a new `Vec` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `Vec`.
/// - With elements, it creates a `Vec` initialized with the provided elements.
///
/// # Examples
/// ```
/// use std_macro_extensions::*;
/// let empty_vector: Vec<i32> = vector!();
/// let numbers = vector!(1, 2, 3);
/// ```
#[macro_export]
macro_rules! vector {
    () => {
        std::vec::Vec::new()
    };
    ($($elem:expr),*) => {
        std::vec::Vec::from([$($elem),*])
    };
}
