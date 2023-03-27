// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10  {
//             println!("Hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     handle.join().unwrap();
//
//     for i in 1..5 {
//         println!("Hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//
//     // handle.join().unwrap();
// }

// fn main() {
//     let v = vec![1, 2, 3];
//
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });
//
//     handle.join().unwrap();
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//     });
//
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// fn main() {
//     // let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//
//     let a = vec![
//         String::from("Hi"),
//         String::from("My"),
//         String::from("Name"),
//         String::from("is"),
//         String::from("Lucas"),
//     ];
//
//     for x in a {
//         println!("{}", x);
//         thread::sleep(Duration::from_secs(1));
//     }
// }

// use std::sync::Mutex;
//
// fn main() {
//     let m = Mutex::new(5);
//     {
//         let mut num = m.lock().unwrap();
//         *num = 6;
//     }
//
//     println!("m = {:?}", m);
// }

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
