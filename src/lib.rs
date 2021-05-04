//! If `var` is a Box, this crate lets you use `var.unbox_ref()` instead of `&*var` for better readability.
//! 
//! Instead of 
//! ```rust
//! let box = Box::new("Hello, Box!".to_owned());
//! let ref_to_value = &*box;
//! ```
//! this crate allows you to write
//! ```rust
//! let box = Box::new("Hello, Box!".to_owned());
//! let ref_to_value = box.unbox_ref();
//! ```
//! which can be easier to read and might help you avoid errors.
//! 
//! To use the crate, just import
//! ```rust
//! use unbox_box::BoxExt as _;
//! ```
//! and the methods will be available to any Box variable within your scope.

#![deny(
    deprecated_in_future,
    exported_private_dependencies,
    future_incompatible,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    private_in_public,
    rust_2018_compatibility,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_qualifications,
    trivial_casts,
    trivial_numeric_casts,
    unused_crate_dependencies,
    unused_lifetimes,
    variant_size_differences
)]
#![warn(clippy::pedantic)]


use core::ops::{Deref, DerefMut};

/// Extends `Box` with the `.unbox_ref()` and `.unbox_mut()` methods.
pub trait BoxExt<T: ?Sized>: Deref<Target = T> + DerefMut<Target = T>{
    /// Returns a reference to the data stored in the `Box` element.
    #[allow(clippy::explicit_deref_methods)]
    fn unbox_ref(&self) -> &T {
        &**self
    }

    /// Returns a mutable reference to the data stored in the `Box` element.
    #[allow(clippy::explicit_deref_methods)]
    fn unbox_mut(&mut self) -> &mut T {
        &mut **self
    }
}

impl<T: ?Sized> BoxExt<T> for Box<T> {}


#[cfg(test)]
mod tests {
    use assert2::assert;
    use super::*;

    /// Try to unbox references to data on the heap.
    #[test]
    fn unbox_heap_box() {
        use std::error::Error;
        let heap_box_1: Box<String> = Box::new("String".to_owned());
        let heap_box_2: Box<Vec<i32>> = Box::new(vec![]);
        let heap_box_3: Box<dyn Error> = "error".into();

        assert!(heap_box_1.unbox_ref() == "String");
        assert!(heap_box_2.unbox_ref() == &vec![]);
        assert!(heap_box_3.unbox_ref().to_string() == "error".to_owned());
    }

    /// Try to unbox data on the stack. 
    #[test]
    fn unbox_stack_box() {
        use std::ops::RangeTo; // RangeTo is Copy, but Range is not

        let stack_box_1: Box<i32> = Box::new(42);
        let stack_box_2: Box<RangeTo<i32>> = Box::new(..10); 
        let stack_box_3: Box<fn() -> i32> = Box::new(|| 5);

        assert!(stack_box_1.unbox_ref() == &42);
        assert!(stack_box_2.unbox_ref() == &(..10));
        assert!(stack_box_3.unbox_ref()() == 5);
    }
}
