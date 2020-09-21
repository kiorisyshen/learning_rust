fn main() {
    // Dereferencing a Raw Pointer
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calling an Unsafe Function or Method
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    // let r = &mut v[..];
    let (a, b) = split_at_mut(&mut v, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Must call foreign function in unsafe
    // abs(3);
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Static mutable variable
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Foreign Function Interface (FFI)
extern "C" {
    fn abs(input: i32) -> i32;
}

// do not mangle function name
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// unsafe traits
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq)]
struct PointTest {
    x: i32,
    y: i32,
}

impl PartialOrd for PointTest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for PointTest {
    fn cmp(&self, other: &PointTest) -> Ordering {
        self.x.cmp(&other.x)
    }
}

// impl PartialEq for points {
//     fn eq(&self, other: &points) -> bool {
//         self.x == other.x
//     }
// }
