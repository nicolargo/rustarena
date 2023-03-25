fn main() {
    no_parameter_no_return();
    parameters_no_return(42);
    println!("Return code: {}", parameters_and_return(42));
    println!("42 + 43 = {}", add(42, 43));
}

fn no_parameter_no_return() {
    println!("A simple function with no parameter and no return");
}

fn parameters_no_return(p: i32) {
    println!("A simple function with parameter {p} (no return)");
}

fn parameters_and_return(p: i32) -> bool {
    println!("A simple function with parameter {p} and return a bool");
    return true;
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}