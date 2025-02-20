/// Creates a new `String` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `String`.
/// - With a string literal or expression, it creates a `String` initialized with the provided value.
#[macro_export]
macro_rules! string {
    () => {
        std::string::String::new()
    };
    ($s:expr) => {
        std::string::String::from($s)
    };
}
