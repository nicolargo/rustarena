fn main() {
    // let foo = 42;
    // println!("{foo}");
    // foo = 7; // error: cannot assign twice to immutable variable

    let mut bar = 42;
    println!("{bar}");
    bar = 7; // ok
    println!("{bar}");
}
