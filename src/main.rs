fn main() {
    use std::io;
    use std::{i32};

    println!("Enter with the First Number");
    let mut input_number1 = String::new();
    io::stdin().read_line(&mut input_number1).expect("Failed to read number");

    println!("Enter with the Second Number");
    let mut input_number2 = String::new();
    io::stdin().read_line(&mut input_number2).expect("Failed to read number");

    let i_number1:i32 = input_number1.trim().parse().ok().expect("The Input 1 it's not a number");
    let i_number2:i32 = input_number2.trim().parse().ok().expect("The Input 2 it's not a number");

    println!("The sum is: {}", i_number1+i_number2);
}
