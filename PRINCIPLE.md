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
let array_of_42 : [i32; 5] = [42, 42, 42, 42, 42]; // Array of 5 i32, initialized with 42
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

"Flow control conditions should be a bool."

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
