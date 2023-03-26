fn main() {
    // Rule 1: Each value in Rust has a variable that’s called its owner.
    let foo = 42; // foo is allocated on the stack
    let foo_s = String::from("Foo"); // foo_s is allocated on the heap (because its size is not known at compile time)

    // Rule 2: There can only be one owner at a time.
    let mut bar_s = foo_s; // bar_s is allocated on the heap and become the owner of the foo_s value
    bar_s.push_str(" and Bar"); // bar_s is modified
    println!("{}", bar_s); // Foo and Bar
    // println!("{}", foo_s); // Error: foo_s is not known anymore (Error: value borrowed here after move)

    // Rule 3: When the owner goes out of scope, the value will be dropped.
    {
        let bar = 43; // bar is allocated on the stack
        println!("{}", bar); // 43
    } // bar is automatically dropped here (memory is freed)
    println!("{}", foo); // foo is still here
    // println!("{}", bar); // Error: bar is not known anymore (Error: not found in this scope)

    // Copy can be used when the size is known at compile time
    // So copy exist for integers, booleans, floats, characters, tuples (if all elements are copy)
    let bar = foo; // bar is allocated on the stack and contains a copy of the foo value
    assert_eq!(foo, bar);
    println!("{}", bar); // 42
}
