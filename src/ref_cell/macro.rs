/// Creates a new `RefCell` instance.
///
/// This macro takes a value and wraps it in a `RefCell`, providing interior mutability with dynamic borrow checking.
/// `RefCell` allows mutable access to its contents even when it is shared among multiple references.
///
/// # Examples
/// ```
/// use std_macro_extensions::*;
/// let my_refcell = refcell!(5);
/// ```
#[macro_export]
macro_rules! refcell {
    ($val:expr) => {
        std::cell::RefCell::new($val)
    };
}
