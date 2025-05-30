/// Invoke a function with a primary array argument and optional additional arguments.
///
/// # Parameters
/// - `path`: The path to the function to invoke.
/// - `array`: The primary argument, typically a slice or vector.
/// - `arg`: (Optional) Additional arguments passed to the function.
///
/// # Returns
/// - The return value of the invoked function.
#[macro_export]
macro_rules! execute {
    ($path: path, $array: expr) => {
        $path($array)
    };
    ($path: path, $array: expr, $($arg: expr), *) => {
        $path($array, $($arg), *)
    };
}
