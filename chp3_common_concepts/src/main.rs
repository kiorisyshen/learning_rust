const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // shadowing
    {
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }

    println!("Hello, world! {THREE_HOURS_IN_SECONDS}");

    another_function(5, "hello func!".to_string());

    // expression
    let y = {
        let x = 3;
        x + 1 // without semicolon, it is a return
    };

    println!("The value of y is: {y}");

    {
        // return value from loop
        let mut counter = 0;
        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("Returned value from loop is: {result}");
    }

    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up; // loop label
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }

    {
        // loop through list
        let a = [10, 20, 30, 40, 50];
        for element in a {
            println!("the value is: {element}");
        }

        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }

    {
        #[derive(Debug)]
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let c = Color(1, 0, 3);
        // let p: Point = Color(0, 0, 0);
        let p: Point = Point(0, 0, 0);

        println!("struct c ele 0: {}, p ele 0: {}", c.0, p.0);
        println!("rect1 is {:?}", c);
    }

    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
            // #[allow(dead_code)]
            // #[cfg(test)]
            fn width(&self) -> bool {
                // self.width > 0
                todo!();
            }
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 width: {}, height: {}", rect1.width, rect1.height);
        println!("rect1 is {:?}", rect1);
        dbg!(&rect1);
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
        if rect1.width() {
            println!("The rectangle has a nonzero width; it is {}", rect1.width);
        }
    }
}

fn another_function(x: i32, label: String) {
    println!("Value of x: {x} with label: {label}");
}
