fn main() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // let f: Thunk = Box::new(|| println!("hi"));
    let f = returns_long_type();
    takes_long_type(f);

    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!(
        "You've randomly chosen an animal, and it says {}",
        animal.noise()
    );

    type Result<T> = std::result::Result<T, std::io::Error>;
}

type Thunk = Box<dyn Fn() + Send + 'static>;
fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
    Box::new(|| ())
}

// Usage of dyn keywords to indicate type is located in heap
struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

// Function never return use !
fn bar() -> ! {
    // --snip--
    panic!();
}

fn bar1() -> ! {
    print!("forever ");
    loop {
        print!("and ever ");
        // break; // cant use break because the return type is !
    }
}

// Default T is sized
// fn generic<T: Sized>(t: T) {
//     // --snip--
// }
// Use ? to make T do not a known size at compile time
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
