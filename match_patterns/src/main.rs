// fn main() {
//     let favorite_color: Option<&str> = None;
//     let is_tuesday = false;
//     let age: Result<u8,_> = "34".parse();
//
//     if let Some(color) = favorite_color  {
//         println!("Using your favorite color, {color}, as the background");
//     } else if is_tuesday {
//         println!("Tuesdy is green day!");
//     } else if let Ok(age) = age {
//         if age > 30 {
//             println!("Using purple as the background color");
//         } else {
//             println!("Using orange as the background color");
//         }
//     } else {
//         println!("Using blue as the background color");
//     }
// }

// fn main() {
//     // let mut stack = Vec::new();
    //
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);
    //
    // while let Some(top) = stack.pop()  {
    //     println!("{}", top);
    // }

//     let v = vec!['a', 'b', 'c'];
//
//     for (index, value) in v.iter().enumerate()  {
//         println!("{}, is at index {}", value, index);
//     }
// }

// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current location: {} {}", x, y);
// }
//
// fn main() {
//     let point = (3, 5);
//     print_coordinates(&point);
// }

// fn main() {
    // let x = 1;
    //
    // match x {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     3 => println!("three"),
    //     _ => println!("anything"),
    // }

    // let x = 5;
    //
    // match x {
    //     1..=5 => println!("One through five"),
    //     _ => println!("something else"),
    // }
    //
    // let y = 'c';
    //
    // match y {
    //     'a'..='j' => println!("early ASCII letter"),
    //     'k'..='z' => println!("late ASCII letter"),
    //     _ => println!("something else"),
    // }
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let p = Point { x: 0, y: 7 };

    // let Point { x: a, y: b } = p;
    // let Point { x, y } = p;
    // assert_eq!(0, a);
    // assert_eq!(7, b);
    
//     match p {
//         Point { x, y: 0 } => println!("On the x axis at {x}"),
//         Point { x: 0, y } => println!("On the y axis at {y}"),
//         Point { x, y } => {
//             println!("On neither axis: ({x}, {y})");
//         } 
//     }
// }

// #[allow(unused)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {
//     // let msg = Message::ChangeColor(0, 160, 255);
//     // let msg = Message::Write(String::from("lucas"));
//     // let msg = Message::Quit;
//     let msg = Message::Move { x: 10, y: 5 };
//
//     match msg {
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure.");
//         }
//         Message::Move { x, y } => {
//             println!("Move in the x direction {x} and in the y direction {y}");
//         }
//         Message::Write(text) => {
//             println!("Text message: {text}");
//         }
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {r}, green {g}, and blue {b}");
//         }
//     }
// }

// #[allow(unused)]
// enum Color {
//     Rgb(i32, i32, i32),
//     Hsv(i32, i32, i32),
// }
//
// #[allow(unused)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(Color),
// }
//
// fn main() {
//     let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
//
//     match msg {
//         Message::ChangeColor(Color::Rgb(r, g, b)) => {
//             println!("Change color to red {r}, green {g}, and blue {b}");
//         }
//         Message::ChangeColor(Color::Hsv(h, s, v)) => {
//             println!("Change color to hue {h}, saturation {s}, value {v}")
//         }
//         _ => (),
//     }
// }

// fn foo(_: i32, y: i32) {
//     println!("This code only uses the y parameter: {}", y);
// }
//
// fn main() {
//     foo(3, 4);
// }

// fn main() {
//     let numbers = (2, 4, 8, 16, 32);
//
//     match numbers {
//         (first, _, third, _, fifth) => {
//             println!("Some numbers: {first}, {third}, {fifth}")
//         }
//     }
//
// }

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0};

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
