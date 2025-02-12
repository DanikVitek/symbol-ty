//! A type-level string, which is a string that is encoded in the type system.

#![no_std]

#[cfg(feature = "unstable__deref")]
use core::{any::TypeId, ops::Deref};
use core::{fmt, fmt::Write, hash::Hash, iter::FusedIterator};

pub use symbol_ty_macro::Symbol;

/// A single character of the symbol, followed by the rest of the symbol.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cons<const C: char, Tail>(Tail);

/// The end of the symbol.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;

#[cfg(not(feature = "unstable__deref"))]
pub trait SymbolBounds: fmt::Display + fmt::Debug + Default + Eq + Ord + Copy + Hash {}
#[cfg(not(feature = "unstable__deref"))]
impl<T> SymbolBounds for T where T: fmt::Display + fmt::Debug + Default + Eq + Ord + Copy + Hash {}

#[cfg(feature = "unstable__deref")]
pub trait SymbolBounds:
    'static
    + fmt::Display
    + fmt::Debug
    + Default
    + Eq
    + Ord
    + Copy
    + Hash
    + Deref<Target = str>
    + AsRef<str>
{
}
#[cfg(feature = "unstable__deref")]
impl<T> SymbolBounds for T where
    T: 'static
        + fmt::Display
        + fmt::Debug
        + Default
        + Eq
        + Ord
        + Copy
        + Hash
        + Deref<Target = str>
        + AsRef<str>
{
}

/// A symbol, which is a type-level string.
pub trait Symbol: SymbolBounds {
    const LEN: usize;

    type Chars: Iterator<Item = char>;

    /// Get an instance of the symbol.
    fn new() -> Self;

    /// Get an iterator over the characters of the symbol.
    fn chars() -> Self::Chars;
}

impl Symbol for Nil {
    const LEN: usize = 0;

    type Chars = core::iter::Empty<char>;

    #[inline(always)]
    fn new() -> Self {
        Self::new()
    }

    #[inline(always)]
    fn chars() -> Self::Chars {
        core::iter::empty()
    }
}

impl<const C: char, Tail: Symbol> Symbol for Cons<C, Tail> {
    const LEN: usize = char_utf8_len(C) + Tail::LEN;

    type Chars = Chars<C, <Tail as Symbol>::Chars>;

    fn new() -> Self {
        Self(Tail::new())
    }

    fn chars() -> Self::Chars {
        Chars {
            used_c: false,
            tail: Tail::chars(),
        }
    }
}

const fn char_utf8_len(c: char) -> usize {
    match c as u32 {
        0..=127 => 1,
        0x80..=0x7FF => 2,
        0x800..=0xFFFF => 3,
        0x10000..=u32::MAX => 4,
    }
}

impl<const C: char, Tail: Symbol> Cons<C, Tail> {
    #[inline(always)]
    pub fn new() -> Self {
        <Self as Symbol>::new()
    }
}

impl Nil {
    #[inline(always)]
    pub const fn new() -> Self {
        Self
    }
}

impl<const C: char, Tail: Symbol> Default for Cons<C, Tail> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Nil {
    #[inline(always)]
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

#[cfg(feature = "unstable__deref")]
impl Deref for Nil {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        ""
    }
}

#[cfg(feature = "unstable__deref")]
struct SymbolStore {
    dereferenced_symbols: once_cell::unsync::Lazy<hashbrown::HashMap<TypeId, &'static str>>,
}

#[cfg(feature = "unstable__deref")]
static SYMBOL_STORE: parking_lot::FairMutex<SymbolStore> =
    parking_lot::const_fair_mutex(SymbolStore {
        dereferenced_symbols: once_cell::unsync::Lazy::new(hashbrown::HashMap::new),
    });

#[cfg(feature = "unstable__deref")]
impl<const C: char, Tail: Symbol + 'static> Deref for Cons<C, Tail> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        let type_id = TypeId::of::<Self>();
        SYMBOL_STORE
            .lock()
            .dereferenced_symbols
            .entry(type_id)
            .or_insert_with(|| {
                extern crate alloc;
                use alloc::{boxed::Box, string::String};

                let mut buf = String::with_capacity(Self::LEN);
                write!(&mut buf, "{self}").unwrap();
                Box::leak::<'static>(buf.into_boxed_str())
            })
    }
}

#[cfg(feature = "unstable__deref")]
impl AsRef<str> for Nil {
    fn as_ref(&self) -> &str {
        ""
    }
}

#[cfg(feature = "unstable__deref")]
impl<const C: char, Tail: Symbol + 'static> AsRef<str> for Cons<C, Tail> {
    fn as_ref(&self) -> &str {
        self
    }
}

#[cfg(feature = "unstable__deref")]
pub fn free<S: 'static>() {
    extern crate alloc;
    use alloc::boxed::Box;

    let type_id = TypeId::of::<S>();
    if let Some(symbol) = SYMBOL_STORE.lock().dereferenced_symbols.remove(&type_id) {
        _ = unsafe { Box::from_raw(symbol as *const str as *mut str) };
    }
}

#[cfg(feature = "unstable__deref")]
pub fn free_ref<S: 'static>(_: &S) {
    free::<S>();
}

#[cfg(feature = "unstable__deref")]
pub fn free_all() {
    extern crate alloc;
    use alloc::boxed::Box;

    for (_, symbol) in SYMBOL_STORE.lock().dereferenced_symbols.drain() {
        _ = unsafe { Box::from_raw(symbol as *const str as *mut str) };
    }
}

/// Iterator over the characters of a given symbol.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Chars<const C: char, Tail> {
    used_c: bool,
    tail: Tail,
}

impl<const C: char, Tail: Iterator<Item = char>> Iterator for Chars<C, Tail> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.used_c {
            self.tail.next()
        } else {
            self.used_c = true;
            Some(C)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.tail.size_hint();
        if self.used_c {
            return (lower, upper);
        }
        (
            lower.saturating_add(1),
            upper.and_then(|upper| upper.checked_add(1)),
        )
    }
}

impl<const C: char, Tail: DoubleEndedIterator<Item = char>> DoubleEndedIterator for Chars<C, Tail> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.tail.next_back() {
            return Some(c);
        }
        if self.used_c {
            None
        } else {
            self.used_c = true;
            Some(C)
        }
    }
}

impl<const C: char, Tail: ExactSizeIterator<Item = char>> ExactSizeIterator for Chars<C, Tail> {
    fn len(&self) -> usize {
        let len = self.tail.len();
        if self.used_c {
            len
        } else {
            len.saturating_add(1)
        }
    }
}

impl<const C: char, Tail: FusedIterator<Item = char>> FusedIterator for Chars<C, Tail> {}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::string::{String, ToString};

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

    #[test]
    fn test_iter() {
        insta_assert!(<Symbol!("hello")>::chars().collect::<String>());
    }

    #[test]
    fn test_iter_rev() {
        insta_assert!(<Symbol!("hello")>::chars().rev().collect::<String>());
    }

    #[test]
    fn test_fuse() {
        let mut chars = <Symbol!("abc")>::chars();
        assert_eq!(chars.next(), Some('a'));
        assert_eq!(chars.next(), Some('b'));
        assert_eq!(chars.next(), Some('c'));
        assert_eq!(chars.next(), None);
        assert_eq!(chars.next(), None);
        assert_eq!(chars.next(), None);
    }

    #[test]
    fn test_rev_fuse() {
        let mut chars = <Symbol!("abc")>::chars().rev();
        assert_eq!(chars.next(), Some('c'));
        assert_eq!(chars.next(), Some('b'));
        assert_eq!(chars.next(), Some('a'));
        assert_eq!(chars.next(), None);
        assert_eq!(chars.next(), None);
        assert_eq!(chars.next(), None);
    }

    #[test]
    fn mem_size_zero() {
        assert_eq!(size_of::<Symbol!("")>(), 0);
        assert_eq!(size_of::<Symbol!("foo_bar")>(), 0);
        assert_eq!(size_of::<Symbol!("foo bar baz")>(), 0);
    }

    #[cfg(feature = "unstable__deref")]
    mod deref {
        use std::sync::atomic::AtomicBool;

        use super::*;
        use crate::{free, free_all, free_ref, SYMBOL_STORE};

        static BUSY: AtomicBool = AtomicBool::new(false);

        fn run_test<F: FnOnce()>(f: F) {
            while BUSY.swap(true, std::sync::atomic::Ordering::SeqCst) {
                std::thread::yield_now();
            }
            f();
            free_all();
            BUSY.store(false, std::sync::atomic::Ordering::SeqCst);
        }

        #[test]
        fn test_deref() {
            run_test(|| {
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 0);

                assert_eq!(&*<Symbol!("hello")>::new(), "hello");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                assert_eq!(&*<Symbol!("hello")>::new(), "hello");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                assert_eq!(&*<Symbol!("hello")>::new(), "hello");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                assert_eq!(&*<Symbol!("hello")>::new(), "hello");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);

                std::thread::scope(|s| {
                    assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                    assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);

                    s.spawn(|| {
                        assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                        assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);
                        assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                        assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);
                    });

                    assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                    assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);
                });

                assert_eq!(&*<Symbol!("")>::new(), "");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);
                assert_eq!(&*<Symbol!("")>::new(), "");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);
                assert_eq!(&*<Symbol!("")>::new(), "");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);
                assert_eq!(&*<Symbol!("")>::new(), "");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);
            });
        }

        #[test]
        fn test_free() {
            run_test(|| {
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 0);

                assert_eq!(&*<Symbol!("hello")>::new(), "hello");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);

                free::<Symbol!("hello")>();
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                free::<Symbol!("hi")>();
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 0);
                
                std::thread::scope(|s| {
                    assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                    assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);

                    s.spawn(|| {
                        assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                        assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                        free::<Symbol!("hi")>();
                        assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 0);
                    });

                    assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                    assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                })
            });
        }

        #[test]
        fn test_free_ref() {
            run_test(|| {
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 0);

                fn use_symbol(s: &impl Symbol) {
                    let _s: &str = s;
                    free_ref(s);
                }

                assert_eq!(&*<Symbol!("hello")>::new(), "hello");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                assert_eq!(&*<Symbol!("hi")>::new(), "hi");
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 2);

                use_symbol(&<Symbol!("hello")>::new());
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 1);
                use_symbol(&<Symbol!("hi")>::new());
                assert_eq!(SYMBOL_STORE.lock().dereferenced_symbols.len(), 0);
            });
        }
    }
}
