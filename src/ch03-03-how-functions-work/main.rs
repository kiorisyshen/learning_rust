//! # Doc comment
//! This comment works for crate documentation
fn main() {
    println!("Hello, world!");

    another_function0();
    another_function1(5);
    another_function2(5, 6);

    let mut x = 5;
    println!("The original value of x is: {}", x);
    let y = {
        // let x = 3;
        x = 3;
        x + 1
    };
    println!("The value of y is: {}, x is: {}", y, x);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

/// Documentation Comments
/// 
/// # Usage
/// This three slashes support markdown notation
fn another_function0() {
    println!("Another function."); // And this is a common comment
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function2(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // return x + 1;
    x + 1
}