/// A macro to create a new `BTreeMap`, providing two usage options:
///
/// 1. `b_tree_map!()` - Creates an empty `BTreeMap`.
/// 2. `b_tree_map!(key1 => value1, key2 => value2, ...)` - Creates a `BTreeMap` containing the specified key-value pairs.
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
