/// Creates a new `HashMap` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `HashMap`.
/// - With key-value pairs, it creates a `HashMap` and inserts the provided pairs into it.
///
/// # Examples
/// Creating an empty `HashMap`:
/// ```
/// let my_map = map!();
/// assert!(my_map.is_empty());
/// ```
///
/// Creating a `HashMap` with key-value pairs:
/// ```
/// use std_macro_extensions::*;
/// let my_map = hash_map!("a" => 1, "b" => 2);
/// assert_eq!(my_map["a"], 1);
/// assert_eq!(my_map["b"], 2);
/// ```
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
