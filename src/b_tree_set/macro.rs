/// Creates a new `BTreeSet<T>`.
///
/// This macro provides two ways to initialize a `BTreeSet`:
///
/// 1. **Empty Set**:
///    - Calling `b_tree_set!()` creates an empty `BTreeSet`.
///
/// 2. **With Elements**:
///    - You can also initialize a `BTreeSet` with elements by providing a comma-separated list of values, e.g., `b_tree_set!(1, 2, 3)`.
///    - This will create a `BTreeSet` containing the specified elements.
#[macro_export]
macro_rules! b_tree_set {
    () => {
        std::collections::BTreeSet::new()
    };
    ($($elem:expr),*) => {{
        let mut set = std::collections::BTreeSet::new();
        $( set.insert($elem); )*
        set
    }};
}
