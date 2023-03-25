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
```

[Code example](./principle04/src/main.rs)


