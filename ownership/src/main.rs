// fn main() {
//     // {
//     //     let s = "hello";
//     // }
//     // println!("{}", s)
//     lca();
//     str();
//     integer();
// }
//
// fn lca() {
//     let mut s = String::from("hello");
//
//     s.push_str(", world!");
//
//     println!("{}", s);
// }
//
// fn str() {
//    let s1 = String::from("hello");
//    let s2 = s1.clone();
//
//    println!("{}, {}", s1, s2);
// }
//
// #[allow(unused_variables)]
// fn integer() {
//    let x = 5; 
//    let y = x; 
//
//     let s: &str = "ksadlsajdkl";
//
//    println!("{}, {}", x, s);
// }

// fn main() {
//     let s = String::from("hello");  // s comes into scope
//
//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here
//
//     let x = 5;                      // x comes into scope
//
//     makes_copy(x);                  // x would move into the function,
//                                     // but i32 is Copy, so it's okay to still
//                                     // use x afterward
//
// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.
//
// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.
//
// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = String::from("Palmeiras");
//
//     let (s2, len) = calculate_lenght(s1);
//
//     println!("The lenght of '{}' is {}.", s2, len);
// }
//
// fn calculate_lenght(s: String) -> (String, usize) {
//     let lenght = s.len(); // len() returns the lenght of a string
//
//     (s, lenght)
// }

// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_lenght(&s1);
//
//     println!("The lenght of '{}' is {}.", s1, len);
//
//     let mut s = String::from("hello");
//
//     change(&mut s);
//
//     println!("The value of '{}'", s);
// }
//
// fn calculate_lenght(s: &String) -> usize {
//    s.len() 
// }
//
// fn change(some_string: & mut String) {
//    some_string.push_str(", world!");
// }

// fn main() {
//     let reference_to_nothing = dangle();
//
//     println!("{}", reference_to_nothing);
// }
// fn dangle() -> String {
//    let s = String::from("hello");
//
//    s
// }

// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s);
//
//     s.clear();
// }
//
// fn first_word(s: &String) -> &str {
//    let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate()  {
//         if item == b' ' {
//            return i;
//         }
//     }
//
//     &s[..]
// }

// fn main() {
//     let s = String::from("hello world");
//
//     let len = s.len();
//
//     let slice = &s[3..len];
//     let slice = &s[..];
// }

fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
