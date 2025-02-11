use core::fmt;

use symbol_ty::Symbol;

fn main() {
    print_symbol(<Symbol!("Hello, world!")>::new());
    println!();
    print_symbol(foo_bar);

    assert_eq!(
        <Symbol!("Hello, world!")>::new(),
        <Symbol!("Hello, world!")>::new(),
    );
}

fn print_symbol<S: Symbol>(s: S)
where
    <S as Symbol>::Chars: DoubleEndedIterator,
{
    println!("{}", s);
    println!("{:?}", s.to_string());
    println!("{:?}", s);
    println!("{:?}", S::chars().collect::<Vec<_>>());
    println!("{:?}", S::chars().rev().collect::<Vec<_>>());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[allow(non_camel_case_types)]
struct foo_bar;

impl Symbol for foo_bar {
    type Chars = Chars_foo_bar;

    fn new() -> Self {
        Self
    }

    fn chars() -> Self::Chars {
        Chars_foo_bar {
            pos: 0,
            end: "foo_bar".len(),
        }
    }
}

impl fmt::Display for foo_bar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("foo_bar")
    }
}

#[allow(non_camel_case_types)]
struct Chars_foo_bar {
    pos: usize,
    end: usize,
}

impl Iterator for Chars_foo_bar {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let res = *b"foo_bar"[..self.end].get(self.pos)? as char;
        self.pos += 1;
        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for Chars_foo_bar {
    fn len(&self) -> usize {
        self.end - self.pos
    }
}

impl DoubleEndedIterator for Chars_foo_bar {
    fn next_back(&mut self) -> Option<Self::Item> {
        let res = *b"foo_bar"[..self.end - self.pos].get(self.end.checked_sub(1)?)? as char;
        self.end -= 1;
        Some(res)
    }
}
