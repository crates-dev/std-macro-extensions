/// Creates a new `Vec` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `Vec`.
/// - With elements, it creates a `Vec` initialized with the provided elements.
#[macro_export]
macro_rules! vector {
    () => {
        std::vec::Vec::new()
    };
    ($($elem:expr),*) => {
        std::vec::Vec::from([$($elem),*])
    };
}
