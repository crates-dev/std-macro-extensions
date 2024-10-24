/// Creates a new `HashSet` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `HashSet`.
/// - With elements, it creates a `HashSet` and inserts the provided elements into it.
///
/// # Examples
/// Creating an empty `HashSet`:
/// ```
/// use std_macro_extensions::*;
/// let my_set = hash_set!();
/// assert!(my_set.is_empty());
/// ```
///
/// Creating a `HashSet` with elements:
/// ```
/// use std_macro_extensions::*;
/// let my_set = hash_set!(1, 2, 3);
/// assert!(my_set.contains(&1));
/// assert!(my_set.contains(&2));
/// assert!(my_set.contains(&3));
/// ```
#[macro_export]
macro_rules! hash_set {
    () => {
        std::collections::HashSet::new()
    };
    ($($elem:expr),*) => {{
        let mut set = std::collections::HashSet::new();
        $( set.insert($elem); )*
        set
    }};
}
