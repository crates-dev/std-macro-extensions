/// A macro to create a new `BTreeMap`, providing two usage options:
///
/// 1. `b_tree_map!()` - Creates an empty `BTreeMap`.
/// 2. `b_tree_map!(key1 => value1, key2 => value2, ...)` - Creates a `BTreeMap` containing the specified key-value pairs.
///
/// # Examples:
///
/// ```rust
/// use std_macro_extensions::*;
/// let empty_map: BTreeMap<i32, i32> = b_tree_map!(); // Creates an empty BTreeMap
///
/// let populated_map = b_tree_map!(
///     "key1" => 1,
///     "key2" => 2,
///     "key3" => 3
/// ); // Creates a BTreeMap with specified key-value pairs
/// ```
///
/// `BTreeMap` is an ordered map that allows iteration over keys in their sorted order. This macro simplifies the
/// process of creating `BTreeMap`, improving code readability and maintainability.
#[macro_export]
macro_rules! b_tree_map {
    () => {
        std::collections::BTreeMap::new()
    };
    ($($key:expr => $val:expr),*) => {{
        let mut map = std::collections::BTreeMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}
