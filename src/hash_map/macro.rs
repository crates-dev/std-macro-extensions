/// Creates a new `HashMap` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `HashMap`.
/// - With key-value pairs, it creates a `HashMap` and inserts the provided pairs into it.
#[macro_export]
macro_rules! hash_map {
    () => {
        std::collections::HashMap::new()
    };
    ($($key:expr => $val:expr),*) => {{
        let mut map = std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}
