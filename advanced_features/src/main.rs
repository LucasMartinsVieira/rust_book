// fn main() {
// let mut num = 5;
//
// let r1 = &num as *const i32;
// let r2 = &mut num as *mut i32;
//
// unsafe {
//     println!("r1 is: {}", *r1);
//     println!("r2 is: {}", *r2);
// }

// unsafe fn dangerous() {}
//
// unsafe {
//     dangerous();
// }

//     let mut v = vec![1, 2, 3, 4, 5, 6];
//
//     let r = &mut v[..];
//
//     let (a, b) = r.split_at_mut(3);
//
//     assert_eq!(a, &mut [1, 2, 3]);
//     assert_eq!(b, &mut [4, 5, 6]);
// }
//
// use std::slice;
//
// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     let ptr = values.as_mut_ptr();
//
//     assert!(mid <= len);
//
//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }
// }

// extern "C" {
//     fn abs(input: i32) -> i32;
// }
//
// fn main() {
//     unsafe {
//         println!("Absolute value of -3 according to C: {}", abs(-3));
//     }
// }

// static HELLO_WORLD: &str = "Hello, world!";
//
// fn main() {
//     println!("name is: {}", HELLO_WORLD);
// }

// static mut COUNTER: u32 = 5;
//
// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }
//
// fn main() {
//     add_to_count(3);
//
//     unsafe {
//         println!("COUNTER: {}", COUNTER);
//     }
// }

/// Advanced Traits

// use std::ops::Add;
//
// #[derive(Debug, Copy, Clone, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl Add for Point {
//     type Output = Point;
//
//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }
//
// fn main() {
//     assert_eq!(
//         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 }
//     )
// }

// use std::fmt;
//
// struct Wrapper(Vec<String>);
//
// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }
//
// fn main() {
//     let w = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }

// fn add_one(x: i32) -> i32 {
//     x + 1
// }
//
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }
//
// fn main() {
//     let answer = do_twice(add_one, 5);
//
//     println!("The answer is: {}", answer);
// }
