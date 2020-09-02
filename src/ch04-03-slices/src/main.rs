fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String slices
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // Equal
    let slice = &s[0..2];
    let slice = &s[..2];

    let slice = &s[3..s.len()];
    let slice = &s[3..];

    let slice = &s[0..s.len()];
    let slice = &s[..];

    // Safe first_word
    let mut s = String::from("hello world");
    let word = first_word_safe(&s);
    // s.clear(); // error!
    println!("the first word is: {}", word);

    // String literals
    let s = "Hello, world!";
    println!("the first word is: {}", first_word_safe_wide(s));

    // Array slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    test_array_ownership(a[1]); // Copy?
    println!("After func: {}", a[1]);

    // Tuple
    let s = String::from("hello world");
    let a = (1, s);
    // println!("After func: {}", s); // Error
    test_tuple_ownership(a.1);
    // println!("After func: {}", a.1); // Error
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_safe(s: &String) -> &str {
    first_word_safe_wide(&s[..])
}

fn first_word_safe_wide(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test_array_ownership(ele: i32) {
    println!("{}", ele);
}

fn test_tuple_ownership(ele: String) {
    println!("{}", ele);
}
