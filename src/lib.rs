//! std-macro-extensions
//!
//! A collection of macro extensions for Rust's standard library
//! data structures, simplifying the creation and manipulation of
//! common collections such as HashMap, Vec, and more.

pub(crate) mod r#arc;
pub(crate) mod b_tree_map;
pub(crate) mod b_tree_set;
pub(crate) mod binary_heap;
pub(crate) mod boxed;
pub(crate) mod r#cell;
pub(crate) mod cin;
pub(crate) mod cout;
pub(crate) mod execute;
pub(crate) mod hash_map;
pub(crate) mod hash_set;
pub(crate) mod linked_list;
pub(crate) mod mutex;
pub(crate) mod path;
pub(crate) mod r#rc;
pub(crate) mod ref_cell;
pub(crate) mod rw_lock;
pub(crate) mod string;
pub(crate) mod vector;
pub(crate) mod vector_deque;

#[cfg(test)]
pub(crate) use std::{
    boxed::Box,
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
    sync::{Mutex, MutexGuard, RwLock},
};
