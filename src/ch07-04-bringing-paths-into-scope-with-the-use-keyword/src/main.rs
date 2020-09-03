use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

// --snip--
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};
// --snip--

use std::collections::*; // often used in tests,
                         // but it makes harder to know what path used
