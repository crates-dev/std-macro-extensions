/// Creates a new hash map instance.
///
/// # Arguments
///
/// - `K` - The type of keys in the map.
/// - `V` - The type of values in the map.
///
/// # Returns
///
/// - `HashMap<K, V>` - A new hash map containing the given key-value pairs.
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
