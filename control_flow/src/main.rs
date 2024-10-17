fn main() {
    // if expression
    let age: u16 = 18;

    if age >= 18 {
        println!("You can drive a car");
    } else {
        println!("you cannot drive a car");
    }

    // multiple conditions with else-if
    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 2, 3 or 4");
    }

    // using if in a let statement
    let condition = false;
    let number = if condition {5} else {6};
    // expression in if and else block should be of compatible type
    // let number = if condition {5} else {"six"}; // this will fail due imcompatible types 

    println!("Number: {number}");

}
