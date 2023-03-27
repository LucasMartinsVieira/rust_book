// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
//
//     const Y: u32 = 20;
//
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * Y;
//
//     println!("{}", THREE_HOURS_IN_SECONDS);
// }

// fn main() {
//     let x = 5;
//
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }
//
//     println!("The value of x is: {}", x);
// }

#![allow(unused)]
fn main() {
    let t: bool = true;

    if t == true {
        println!("Lucas")
    } else {
        println!("Thiago")
    }

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    for x in months  {
        println!("{}", x)
    }

    println!("We're on the month of: {}", months[2] )
}
