use symbol_ty::Symbol;

fn main() {
    print_symbol(<Symbol!("Hello, world!")>::new());

    assert_eq!(
        <Symbol!("Hello, world!")>::new(),
        <Symbol!("Hello, world!")>::new(),
    );
}

fn print_symbol(s: impl Symbol) {
    println!("{}", s);
    println!("{:?}", s.to_string());
    println!("{:?}", s);
}
