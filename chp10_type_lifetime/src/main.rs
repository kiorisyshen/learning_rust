use std::cmp::PartialOrd;

// fn largest(list: &[impl PartialOrd]) -> &impl PartialOrd {
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// cargo test -- --test-threads=1
// cargo test -- --show-output // show output even if passed
// cargo test it    // run tests with substring
// cargo test -- --ignored
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // #[test]
    // #[should_panic(expected = "panic msg")] // expected substring
    // fn test_panic() {
    //     panic!("test panic");
    //     // println!("test no panic");
    // }

    #[test]
    fn it_works_2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

trait Summary {
    fn summarize(&self) -> String {
        // default trait
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
// impl Summary for NewsArticle {} // use default trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({}), content: {}",
            self.headline, self.author, self.location, self.content
        )
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}, reply: {}, retweet: {}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        // traits as parameters
        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        // traits bound
        // pub fn notify<T: Summary>(item: &T) {
        //     println!("Breaking news! {}", item.summarize());
        // }

        // multiple traits
        // pub fn notify(item: &(impl Summary + Display)) {}
        // pub fn notify<T: Summary + Display>(item: &T) {}

        // fn some_function<T, U>(t: &T, u: &U) -> i32
        // where
        //     T: Display + Clone,
        //     U: Clone + Debug,
        // {
        // }
    }

    {
        // Dangling references
        // let r;
        // {
        //     let x = 5;
        //     r = &x;
        // }
        // println!("r: {}", r);
    }

    {
        let string1 = String::from("abcd");
        {
            let string2 = "xyz";

            let result = longest(string1.as_str(), string2);
            println!("The longest string is {}", result);
        }

        // let string1 = String::from("long string is long");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str());
        // }
        // println!("The longest string is {}", result);
    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    {
        let s: &'static str = "I have a static lifetime.";
        // All string literals have the 'static lifetime.
        // The text of this string is stored directly in the program’s binary, which is always available.
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
