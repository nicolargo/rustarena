fn main() {
    let tuple = (42, 42.0, true, 'a', "Hello world!"); // Tuple of 5 elements
    let mut array_of_42 : [i32; 5] = [42, 42, 42, 42, 42]; // Array of 5 i32, initialized with 42
    // let mut array_of_42 : [42; 5]; // Array of 5 i32, initialized with 42 (shorter syntax)
    let mut tuple_of_42 = (42, 42, 42, 42, 42); // Tuple of 5 i32, initialized with 42

    dbg!(tuple);
    println!("tuple.0 = {}", tuple.0);
    println!("tuple.4 = {}", tuple.4);

    dbg!(array_of_42);
    println!("array_of_42[0] = {}", array_of_42[0]);
    array_of_42[4] = 43;
    println!("array_of_42[4] = {}", array_of_42[4]);

    dbg!(tuple_of_42);
    println!("tuple_of_42.0 = {}", tuple_of_42.0);
    tuple_of_42.4 = 43;
    println!("tuple_of_42.4 = {}", tuple_of_42.4);
    //tuple_of_42.4 = "Hello"; // error: mismatched types

    // You can not iterate through a tuple
    // But you can iterate through an array
    for (index, value) in array_of_42.iter().enumerate() {
        println!("array_of_42[{index}] = {value}");
    }
}
