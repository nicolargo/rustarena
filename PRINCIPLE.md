# Rust principle #01

"By default variables are immutables."

```rust
let foo = 42; // foo is immutable
let mut bar = 42; // bar is mutable
```

[Code example](./principle01/src/main.rs)

# Rust principle #02

"Constants are never mutables."

```rust
const FOO: u32 = 42; // FOO is a constant (always immutable)
```

[Code example](./principle02/src/main.rs)

# Rust principle #03

"Variables could be shadowed inside a scope."

```rust
let foo = 42;
// foo is 42
{
    let foo = 43; // foo is shadowed by a new variable
    // foo is 43
}
// foo is still 42
```

[Code example](./principle03/src/main.rs)

# Rust principle #04

"Variables are typed."

```rust
let foo = 42; // is an i32 (default type for integer)
let ffoo = 42.0; // is a f64 (default type for float)
let bar: u32 = 42; // is a u32 (unsigned integer on 32 bits)
let fbar: f32 = 42.0; // is a f32 (float on 32 bits)
let vrai = true; // is a bool
let faux: bool = false; // is also a bool
let a: char = 'a'; // is a char
let smile = '😀'; // is also a char (yes unicode is supported)
```

[Code example](./principle04/src/main.rs)

# Rust principle #05

"Tuples can hold elements of different types, while arrays hold elements of the same type. They both have a fixed size."

```rust
let tuple = (42, 42.0, true, 'a', "Hello world!"); // Tuple of 5 elements
let tuple_of_42 = (42, 42, 42, 42, 42); // Tuple of 5 i32, initialized with 42
```

[Code example](./principle05/src/main.rs)

# Rust principle #06

"Functions parameters and return values should be declared."

```rust
```

[Code example](./principle06/src/main.rs)

# Rust principle #07

"Comments Rust code using //. Documents Rust code using ///."

```rust
// This is a comment
let foo = 42; // This is a comment too
// This is a comment on
// multiple lines

/// This is a documentation comment for the bar public function
/// # Arguments
/// * `a` - The first argument
/// * `b` - The second argument
/// # Returns
/// The sum of a and b
/// # Example
/// ```
/// let result = bar(1, 2);
/// assert_eq!(result, 3);
/// ```
pub fn bar(a: i32, b: i32) -> i32 {
    a + b
}
```

[Code example](./principle07/src/main.rs)

# Rust principle #08

"Flow control condition should be a bool."

```rust
if true {
    // This is executed
}

if !false {
    // This is executed
}

let result: bool = if true { 42 } else { 0 };

if result == 42 {
    // This is executed
} else {
    // This is not executed
}

let mut index = 0;
while index < result {
    index += 1;
}
assert_eq!(index, result);
```

[Code example](./principle08/src/main.rs)

# Rust principle #09

"Rust do not have any garbage collector.
Rust do not ask developer to allocate and free memory.
Rust use ownership rules to manage memory."

```rust
// Rule 1: Each value in Rust has a variable that’s called its owner.
//==============================================================
let foo = 42; // foo is allocated on the stack
let foo_s = String::from("Foo"); // foo_s is allocated on the heap (because its size is not known at compile time)

// Rule 2: There can only be one owner at a time.
//==============================================================
let mut bar_s = foo_s; // bar_s is allocated on the heap and become the owner of the foo_s value
bar_s.push_str(" and Bar"); // bar_s is modified
println!("{}", bar_s); // Foo and Bar
// println!("{}", foo_s); // Error: foo_s is not known anymore (Error: value borrowed here after move)

// Rule 3: When the owner goes out of scope, the value will be dropped.
//==============================================================
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
```

[Code example](./principle09/src/main.rs)

# Rust principle #10

"Reference is a type of pointer that points to a value in memory without taking ownership of it."

```rust
fn main() {
    let foo_s1 = String::from("Foo 1");
    myfunction(foo_s1); // foo_s1 is moved to the process function
    // println!("{}", foo_s1); // Error: foo_s1 is not known anymore (Error: value borrowed here after move)

    let foo_s2 = String::from("Foo 2");
    myfunction_ref(&foo_s2); // &foo_s2 is a reference to the foo_s2 value
    println!("{}", foo_s2); // foo_s2 is still here

    let mut foo_s3 = String::from("Foo 3");
    myfunction_ref_mut(&mut foo_s3); // &mut foo_s3 is a mutable reference to the foo_s3 value
    println!("{}", foo_s3); // foo_s3 is still here
}

fn myfunction(s: String) { // s is allocated on the stack and become the owner of the bar_s value
    println!("{}", s);
}

fn myfunction_ref(s: &String) { // s is allocated on the stack and become the owner of the bar_s value
    println!("{}", s);
}

fn myfunction_ref_mut(s: &mut String) { // s is allocated on the stack and become the owner of the bar_s value
    s.push_str(" and Bar");
    println!("{}", s);
}
```

[Code example](./principle10/src/main.rs)

# Rust principle #11

"Rust has three types to store a sequence of a value: array, vector and slice."

```rust
// Array is stored on the stack (size is known at compile time and can not be changed)
let array_of_42 : [i32; 5] = [42, 42, 42, 42, 42]; // Array of 5 i32, initialized with 42

// Vector is stored on the heap (size is not known at compile time and can be changed)
let mut vector_of_42 : Vec<i32> = vec![42, 42, 42, 42, 42]; // Vector of 5 i32, initialized with 42
vector_of_42.push(42); // Add a new element to the vector

// Slice is a reference to a sequence of values (size is not known at compile time and can not be changed)
// Has slice is a reference, it did not take ownership of the values
let slice_of_42 : &[i32] = &vector_of_42; // Slice of 6 i32, initialized with 42
let slice_first_3 : &[i32] = &vector_of_42[0..3]; // Slice of 3 i32, initialized with 42
```

[Code example](./principle11/src/main.rs)


