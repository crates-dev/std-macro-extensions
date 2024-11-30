# std-macro-extensions

[![](https://img.shields.io/crates/v/std-macro-extensions.svg)](https://crates.io/crates/std-macro-extensions)
[![](https://docs.rs/std-macro-extensions/badge.svg)](https://docs.rs/std-macro-extensions)
[![](https://img.shields.io/crates/l/std-macro-extensions.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/std-macro-extensions/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/std-macro-extensions/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/STD-MACRO-EXTENSIONS/)

A collection of macro extensions for Rust's standard library data structures, simplifying the creation and manipulation of common collections such as `HashMap`, `Vec`, and more.

## Features

- **Simplified Initialization**: Use macros to easily create instances of common data structures.
- **Supports Various Data Structures**: Includes macros for `Vec`, `HashMap`, `Arc`, and more.
- **Easy to Use**: Intuitive syntax for quick data structure creation.

## Installation

To install `std-macro-extensions` run cmd:

```sh
cargo add std-macro-extensions
```

## Usage

Here are some examples of how to use the macros provided by this crate:

### Example: Using `arc!`

```rust
use std_macro_extensions::*;

fn main() {
    let value = arc!(5);
}
```

### Example: Using `b_tree_map!`

```rust
use std_macro_extensions::*;

fn main() {
    let empty_map: BTreeMap<i32, i32> = b_tree_map!();
    let b_tree_map_a: BTreeMap<&str, &str> = b_tree_map!(
        "a" => "a",
        "b" => "b"
    );
}
```

### Example: Using `b_tree_set!`

```rust
use std_macro_extensions::*;

fn main() {
    let empty_set: BTreeSet<i32> = b_tree_set!();
    let number_set: BTreeSet<i32> = b_tree_set!(1, 2, 3);
}
```

### Example: Using `binary_heap!`

```rust
use std_macro_extensions::*;

fn main() {
    let empty_set: BTreeSet<i32> = b_tree_set!();
    let number_set: BTreeSet<i32> = b_tree_set!(1, 2, 3);
}
```

### Example: Using `boxed!`

```rust
use std_macro_extensions::*;

fn main() {
    let boxed_value = boxed!(10);
}
```

### Example: Using `cell!`

```rust
use std_macro_extensions::*;

fn main() {
    let cell_value: Cell<i32> = cell!(5);
}
```

### Example: Using `hash_map!`

```rust
use std_macro_extensions::*;

fn main() {
    let my_map: HashMap<&str, i32> = hash_map!();
    let my_map: HashMap<&str, i32> = hash_map!("a" => 1, "b" => 2);
}
```

### Example: Using `hash_set!`

```rust
use std_macro_extensions::*;

fn main() {
    let my_set: HashSet<i32> = hash_set!();
    let my_set: HashSet<i32> = hash_set!(1, 2, 3);
}
```

### Example: Using `linked_list!`

```rust
use std_macro_extensions::*;

fn main() {
    let my_list: LinkedList<i32> = linked_list!();
    let my_list: LinkedList<i32> = linked_list!(1, 2, 3);
}
```

### Example: Using `mutex!`

```rust
use std_macro_extensions::*;

fn main() {
    let my_mutex: Mutex<i32> = mutex!(5);
    let lock: MutexGuard<'_, i32> = my_mutex.lock().unwrap();
}
```

### Example: Using `rc!`

```rust
use std_macro_extensions::*;

fn main() {
    let my_rc = rc!(5);
}
```

### Example: Using `refcell!`

```rust
use std_macro_extensions::*;

fn main() {
    use std_macro_extensions::*;
    let my_refcell = refcell!(5);
}
```

### Example: Using `rw_lock!`

```rust
use std_macro_extensions::*;

fn main() {
    let my_rwlock = rw_lock!(5);
}
```

### Example: Using `string!`

```rust
use std_macro_extensions::*;

fn main() {
    let empty_string = string!();
    let hello_string = string!("Hello");
}
```

### Example: Using `vector!`

```rust
use std_macro_extensions::*;

fn main() {
    let empty_vector: Vec<i32> = vector!();
    let numbers = vector!(1, 2, 3);
}
```

### Example: Using `vector_deque!`

```rust
use std_macro_extensions::*;

fn main() {
    let empty_deque: VecDeque<i32> = vector_deque!();
    let numbers = vector_deque!(1, 2, 3);
}
```

## Available Macros

- `arc!`: Creates an `Arc<T>`.
- `vector!`: Creates a `Vec<T>`.
- `map!`: Creates a `HashMap<K, V>`.
- `set!`: Creates a `HashSet<T>`.
- `b_tree_map!`: Creates a `BTreeMap<K, V>`.
- `b_tree_set!`: Creates a `BTreeSet<T>`.
- `list!`: Creates a `LinkedList<T>`.
- `heap!`: Creates a `BinaryHeap<T>`.
- `string!`: Creates a `String`.
- `boxed!`: Creates a `Box<T>`.
- `rc!`: Creates an `Rc<T>`.
- `arc!`: Creates an `Arc<T>`.
- `mutex!`: Creates a `Mutex<T>`.
- `rw_lock!`: Creates a `RwLock<T>`.
- `cell!`: Creates a `Cell<T>`.
- `ref_cell!`: Creates a `RefCell<T>`.
- `vector_deque!`: Creates a `VecDeque<T>`.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
