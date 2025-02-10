//! A type-level string, which is a string that is encoded in the type system.

#![no_std]

use core::{fmt, fmt::Write, hash::Hash};

pub use symbol_ty_macro::Symbol;

/// A single character of the symbol, followed by the rest of the symbol.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cons<const C: char, Tail>(Tail);

/// The end of the symbol.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;

/// A symbol, which is a type-level string.
pub trait Symbol: fmt::Display + fmt::Debug + Default + Eq + Ord + Copy + Sized + Hash {
    /// Get an instance of the symbol.
    fn new() -> Self;

    #[doc(hidden)]
    fn _sealed(_: __private::Seal) {}
}

mod __private {
    pub struct Seal;
}

impl Symbol for Nil {
    fn new() -> Self {
        Self
    }
}

impl<const C: char, Tail: Symbol> Symbol for Cons<C, Tail> {
    fn new() -> Self {
        Self(Tail::new())
    }
}

impl<const C: char, Tail: Symbol> Default for Cons<C, Tail> {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Nil {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl<const C: char, Tail: fmt::Display> fmt::Display for Cons<C, Tail> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(C)?;
        self.0.fmt(f)
    }
}

impl<const C: char, Tail: fmt::Debug> fmt::Debug for Cons<C, Tail> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Cons").field(&C).field(&self.0).finish()
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::ToString;

    use insta::{assert_debug_snapshot, with_settings};

    use super::Symbol;

    macro_rules! insta_assert {
       ($e:expr) => {
            with_settings!({prepend_module_to_snapshot => false}, {
                assert_debug_snapshot!($e);
            });
        };
    }

    #[test]
    fn test_empty_symbol() {
        insta_assert!(<Symbol!("")>::new());
    }
    
    #[test]
    fn test_display() {
        insta_assert!(<Symbol!("hello")>::new().to_string());
    }

    #[test]
    fn test_debug() {
        insta_assert!(<Symbol!("hello")>::new());
    }
}
