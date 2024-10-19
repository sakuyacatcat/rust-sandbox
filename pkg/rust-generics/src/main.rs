mod lib;

use lib::{Summary, Tweet};
use std::cmp::PartialOrd;

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // generic function
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest1(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest1(&char_list);
    println!("The largest char is {}", result);

    // generic struct
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    let _p3 = p1.mixup(p2);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            // もちろん、ご存知かもしれませんがね、みなさん
            "of course, as you probably already know, people",
        ),
        reply: false.to_string(),
        retweet: false.to_string(),
    };

    println!("1 new tweet: {}", tweet.summarize());

    // lifetimes
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }

    let string1 = String::from("abcd");
    {
        let string2 = String::from("xyz");
        let result = longest2(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // struct with lifetime
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn largest1<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
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
