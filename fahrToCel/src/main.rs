use std::io;

fn main() {
    let mut fahr_str = String::new();
    io::stdin().read_line(&mut fahr_str)
        .expect("Failed to read number");

    let fahr = fahr_str.trim().parse::<f32>()
        .expect("Error check");

    let cel = (fahr - 32.0) * (5.0/9.0);
    println!("The converted temp in celsius is: {}", cel);
}
