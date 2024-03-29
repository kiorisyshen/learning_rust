//----- Integer -----
// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// arch	isize	usize

// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

fn main() {
    //----- Bool -----
    let t = true;
    let f: bool = false; // with explicit type annotation

    //----- Float -----
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //----- Operations -----
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    //----- Char -----
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //----- Tuple -----
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //----- Array -----
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // fill 3 in array with 5 elements
    let first = a[0];
    let second = a[1];
}