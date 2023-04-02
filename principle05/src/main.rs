fn main() {
    let tuple = (42, 42.0, true, 'a', "Hello world!"); // Tuple of 5 elements
    let mut tuple_of_42 = (42, 42, 42, 42, 42); // Tuple of 5 i32, initialized with 42

    dbg!(tuple);
    println!("tuple.0 = {}", tuple.0);
    println!("tuple.4 = {}", tuple.4);

    dbg!(tuple_of_42);
    println!("tuple_of_42.0 = {}", tuple_of_42.0);
    tuple_of_42.4 = 43;
    println!("tuple_of_42.4 = {}", tuple_of_42.4);
    //tuple_of_42.4 = "Hello"; // error: mismatched types
}
