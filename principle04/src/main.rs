// Source: https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let foo = 42; // is an i32 (default type for integer)
    let ffoo = 42.0; // is a f64 (default type for float)
    let bar: u32 = 42; // is a u32 (unsigned integer on 32 bits)
    let fbar: f32 = 42.0; // is a f32 (float on 32 bits)

    dbg!(foo);
    print_type_of(&foo);
    dbg!(ffoo);
    print_type_of(&ffoo);
    dbg!(bar);
    print_type_of(&bar);
    dbg!(fbar);
    print_type_of(&fbar);
}