fn main() {
    // This is a comment
    let foo = 42; // This is a comment too
    // This is a comment on
    // multiple lines

    println!("{}", bar(foo, 1))
}

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
