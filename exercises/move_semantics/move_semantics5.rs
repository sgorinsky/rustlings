// move_semantics5.rs
// Make me compile without adding, removing, or changing any of the
// lines in `main()`.
// Execute `rustlings hint move_semantics5` for hints :)

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    print_type_of(&y);
    let z = &mut *y;
    *z += 1000;
    assert_eq!(x, 1200);
}
