/// Creates a new `HashMap` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `HashMap`.
/// - With key-value pairs, it creates a `HashMap` and inserts the provided pairs into it.
///
/// # Examples
/// Creating an empty `HashMap`:
/// ```
/// use std_macro_extensions::*;
/// let my_map: HashMap<&str, i32> = hash_map!();
/// ```
///
/// Creating a `HashMap` with key-value pairs:
/// ```
/// use std_macro_extensions::*;
/// let my_map = hash_map!("a" => 1, "b" => 2);
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
