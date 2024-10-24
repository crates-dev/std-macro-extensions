/// Creates a new `Mutex` instance.
///
/// This macro takes a value and wraps it in a `Mutex`, providing thread-safe interior mutability.
/// The `Mutex` type ensures mutual exclusion, allowing only one thread to access the value at a time.
///
/// # Examples
/// ```
/// use std_macro_extensions::*;
/// let my_mutex = mutex!(5);
/// let lock = my_mutex.lock().unwrap();
/// ```
#[macro_export]
macro_rules! mutex {
    ($val:expr) => {
        std::sync::Mutex::new($val)
    };
}
