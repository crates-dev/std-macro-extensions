/// Creates a new `LinkedList` instance.
///
/// This macro can be used in two forms:
/// - Without arguments, it creates an empty `LinkedList`.
/// - With elements, it creates a `LinkedList` and appends the provided elements to the back of the list.
///
/// # Examples
/// Creating an empty `LinkedList`:
/// ```
/// use std_macro_extensions::*;
/// let my_list: LinkedList<i32> = linked_list!();
/// ```
///
/// Creating a `LinkedList` with elements:
/// ```
/// use std_macro_extensions::*;
/// let my_list = linked_list!(1, 2, 3);
/// ```
#[macro_export]
macro_rules! linked_list {
    () => {
        std::collections::LinkedList::new()
    };
    ($($elem:expr),*) => {{
        let mut list = std::collections::LinkedList::new();
        $( list.push_back($elem); )*
        list
    }};
}
