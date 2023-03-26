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

    // At any time, you can have either one mutable reference or any number of immutable references
    let mut foo_s4 = String::from("Foo 4");
    let foo_s4_ref1 = &foo_s4; // immutable reference
    let foo_s4_ref2 = &foo_s4; // immutable reference
    let foo_s4_ref3 = &mut foo_s4; // mutable reference
    //println!("{}, {}, {}", foo_s4_ref1, foo_s4_ref2, foo_s4_ref3); Error: cannot borrow `foo_s4` as mutable because it is also borrowed as immutable
    println!("{}", foo_s4_ref3); // Ok

    // References must always be valid
    let foo_s5 = String::from("Foo 5");
    let foo_s5_ref1 = &foo_s5;
    let foo_s5_ref2 = &foo_s5;
    //let foo_s5_ref3 = &mut foo_s5; // Error: cannot borrow `foo_s5` as mutable because it is also borrowed as immutable
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
