fn main() {
    // let foo = 5;
    // println!("{foo}");
    // foo = 4; // error: cannot assign twice to immutable variable

    let mut bar = 5;
    println!("{bar}");
    bar = 4; // ok
    println!("{bar}");
}
