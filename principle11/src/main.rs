fn main() {
    // Array is stored on the stack (size is known at compile time and can not be changed)
    let array_of_42 = [42; 5];
    // You can not iterate through a tuple but you can iterate through an array
    for (index, value) in array_of_42.iter().enumerate() {
        println!("array_of_42[{index}] = {value}");
    }

    // Vector is stored on the heap (size is not known at compile time and can be changed)
    let mut vector_of_42 : Vec<i32> = vec![42, 42, 42, 42, 42]; // Vector of 5 i32, initialized with 42
    vector_of_42.push(42); // Add a new element to the vector
    for (index, value) in vector_of_42.iter().enumerate() {
        println!("vector_of_42[{index}] = {value}");
    }
    // Vector can be initialized from an iterator
    let vector_to_42 : Vec<i32> = (0..42).collect();
    dbg!(vector_to_42);
    // Vector can be sliced
    dbg!(&vector_of_42[0..2]);

    // Slice is a reference to a sequence of values (size is not known at compile time and can not be changed)
    // Has slice is a reference, it did not take ownership of the values
    let slice_of_42 : &[i32] = &vector_of_42; // Slice of 6 i32, initialized with 42
    for (index, value) in slice_of_42.iter().enumerate() {
        println!("slice_of_42[{index}] = {value}");
    }
    // Slice can be used to access to a part of a vector
    dbg!(&slice_of_42[0..3]); // Vector of 3 i32, initialized with 42
}
