fn main() {
    bigger_num(7, 6);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    let x = y;
    
    println!("The value of y is: {}", x);
}

fn bigger_num(x: i32, y: i32) {
    if x > y {
        println!("{x}");
    } else {
        println!("{y}");
    }
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label)
}
