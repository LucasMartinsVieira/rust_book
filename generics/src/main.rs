/// Generics
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let mut largest = &number_list[0];
//
//     for number in &number_list  {
//         if number > largest {
//             largest = number;
//         }
//     }
//
//     println!("The largest number is: {}", largest);
// }

// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list  {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest(&number_list);
//     println!("The largest number is: {}", result);
//
//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//
//     let result = largest(&number_list);
//     println!("The largest number is: {}", result);
// }

// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest_i32(&number_list);
//     println!("The largest number is: {}", result);
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest_char(&char_list);
//     println!("The largest char is: {}", result);
// }
// #[allow(unused)]
// #[allow(unused)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// #[allow(unused)]
// struct Point2<T, U> {
//     x: T,
//     y: U,
// }
//
// #[allow(unused)]
// fn whatever2() {
//     let both_integer = Point2 { x: 5, y: 10 };
//     let both_float = Point2 { x: 1.0, y: 4.0 };
//     let integer_and_float = Point2 { x: 5, y: 4.0 };
// }
//
// #[allow(unused)]
// fn whatever() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }
//
// impl <T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
//
// fn main() {
//     let p = Point { x: 5, y: 10};
//
//     println!("p.x = {}", p.x());
// }

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1
// }
//
// impl <X1, Y1> Point<X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
//
// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c' };
//
//     let p3 = p1.mixup(p2);
//
//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }

/// Traits

// use generics::{Summary, Tweet};
//
// fn main() {
//     let tweet = Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you pprobably alredy know people"
//         ),
//         reply: false,
//         retweet: false,
//     };
//
//     println!("1 new tweet: {}", tweet.summarize());
// }

/// Lifetimes

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
