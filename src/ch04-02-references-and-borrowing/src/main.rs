fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    // To prevent data race, mutable reference must be only one in one scope
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // Also error because of data race
    // let mut s = String::from("hello");
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);

    // Make sure r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
    // println!("{} and {}", r1, r2); // can not use r1 r2 because of r3

    // let reference_to_nothing = dangle();
    let string_normal = no_dangle();
    println!("{}", string_normal);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
fn no_dangle() -> String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    // &s // we return a reference to the String, s
    s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
