// Source: https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // Numbers
    //========

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

    // let foo_plus_ffoo = foo + ffoo; // is forbiden because Rust doesn't want you to shoot yourself in the foot
    let foo_plus_ffoo = f64::from(foo) + ffoo; // is a f64. Had to use f64::from(foo) to convert foo to a f64

    dbg!(foo_plus_ffoo);
    print_type_of(&foo_plus_ffoo);

    // Boolean
    //========

    let vrai = true;
    let faux = false;

    dbg!(vrai);
    print_type_of(&vrai);
    dbg!(faux);
    print_type_of(&faux);

    // Char
    //=====

    let a = 'a';
    let smile = '😀';

    dbg!(a);
    print_type_of(&a);
    dbg!(smile);
    print_type_of(&smile);

}