fn main() {
    another_function(5);
    third_function(5, 6);
    assignment();
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn third_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of x is: {}", y);
}

fn assignment() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of assingment y is: {}", y);
}