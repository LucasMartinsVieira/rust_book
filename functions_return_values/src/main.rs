#[allow(unused)]
/// Hi, this is a documenting comment!
fn plus_one(x: i32) -> i32 {
    x + 1
}

// NOTE: This is a comment using Nvim-todo-comments by folke.
fn main() {
    let x = plus_one(5);
    less_5(3);

    println!("The value of x is: {}", x);
    
    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
    _while();
    oh_loopinho_meu();
    liftoff();
}

#[allow(unused)]
fn less_5(number: i32) {
    if number < 5 {
        println!("The number is: {}, which is less than 5", number)
    } else {
        println!("The number is: {}, which is greater than 5", number)
    }
}

// This func don't work
// fn number_differ_0() -> i32 {
//     let number = 3;
//
//     if number {
//         println!("{}", number);
//     }
//
// }

#[allow(unused)]
fn oh_loopinho_meu() {
    // let number = [1, 2, 3, 4, 5, 6, 7, 8];
    let number = ["lucas", "thiago", "sislede"];

    for x in number {
        println!("{}", x);
    }

    println!("My name is: {}", number[0]);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);
}

fn liftoff() {
    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{}!", number);
    //
    //     number -= 1;
    // }
    //
    // println!("LIFTOFF!!!")
    
    for number in (1..4).rev()  {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn _while() {
    let a = [10, 20, 30, 40 ,50];

    // let mut index = 0;
    //
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //     index += 1;
    // }

    // This is better than other 
    for element in a  {
        println!("The value is: {}", element);
    }
}
