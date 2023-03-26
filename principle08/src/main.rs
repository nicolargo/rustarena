// use std::ptr;

fn main() {
    let vrai: bool = true;
    let faux: bool = false;

    if vrai {
        println!("True");
    }

    if !faux {
        println!("True");
    }

    // Error on compilation: 1 is not a boolean
    // if 1 {
    //     println!("True");
    // }

    // Error on compilation: null is not a boolean
    // if ptr::null() {
    //     println!("True");
    // }

    let result: i32 = if true { 42 } else { 0 };
    println!("{}", result);
    if result == 42 {
        println!("True");
    } else {
        println!("False"); // Never executed
    }

    let mut index = 0;
    while index < result {
        index += 1;
    }
    assert_eq!(index, result);
    println!("{}", index); // 42
}
