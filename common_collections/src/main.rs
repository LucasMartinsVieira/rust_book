// Vectors
// fn main() {
//     let mut v = vec![1, 2, 3, 4, 5];
//
//     let third: &i32 = &v[2];
//     println!("The third element is: {}", third);
//
//     let third: Option<&i32> = v.get(2);
//     match third {
//         Some(third) => println!("The third element is: {}.", third),
//         None => println!("The is no third element."),
//     }
//
//     for i in &v  {
//         println!("{}", i);
//     }
//
//     for i in &mut v  {
//         *i += 50;
//         println!("{}", i);
//     }
//
// }

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }
//
// fn main() {
//     #[allow(unused_variables)]
//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.22),
//     ];
// }

// Strings
// fn main() {
    // let mut s1 = String::new();
    //
    // let data = "initial contents";
    //
    // let s2 = data.to_string();
    // or
    // let s2 = "initial contents".to_string();
    //
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // println!("{s}");
    //
    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is: {s2} and s1 is: {s1}");
    //
    // let mut s = String::from("lo");
    // s.push('l');
    // println!("{s}");
    //
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World!");
    // let s3 = s1 + &s2;
    // println!("{s3}");
    //
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    //
    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("{s}");
    // let hello = "Здравствуйте";
    // // let size = hello.len();
    // // println!("{size}");
    // let s = &hello[0..4];
    // println!("{s}");
    //
    // for c in "Зд".chars() {
    //     println!("{c}");
    // }
    //
    // for b in "Зд".bytes() {
    //     println!("{b}");
    // }
// }

// Hashmaps
use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // for (key, value) in &scores  {
    //     println!("{}: {}", key, value);
    // }

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    //
    // println!("{:?}", scores);
    
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace()  {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
