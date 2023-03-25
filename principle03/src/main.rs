fn main() {
    let foo = 42;
    println!("{foo}");
    {
        let foo = 43;
        println!("{foo}");
    }
    println!("{foo}");
}
