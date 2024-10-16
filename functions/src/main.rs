// entry point
fn main() {
    hello_world()
}

// hoisting - can call function anywhere in your code
fn hello_world(){
    println!("Hello, World");
    tell_height(100);
    human_id("bob", 34, 108.9);

    // example of expression
    let x = {
        let price = 5;
        let qty = 10;
        price  * qty  // last line , no semicolon needed
        // or return price * qty;
    };
    println!("Result is: {}", x);
    
    let y = add(4,5);
    println!("Value of y is: {}", y);

    // calling BMI function
    let weight = 70.0;
    let height = 1.82;
    let bmi = calculate_bmi(weight, height);
    println!("your bmi is: {:.2}", bmi);
}

// you can insert input values
fn tell_height(height: u32){
    println!("my height is: {} cm", height);
}


// inserting multiple parameters
fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old and my height is {} cm.", name, age, height);
}

// function returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and statements

// Expression: Anything that returns a value
// expression examples
// 5
// true & false
// add(3,4)
// if condition {value1} else {value2}
//({code})

// Statement: Anything that does not return a value
// almost all statements in Rust end with ;
// let y = let x = 10;
// 1 variables declarations: let x = 5;
// 2 function definitions: fn foo() {}
// 3 control flow statements: if condition {code} else {code}, while condition, etc


// BMI = height(kg) / height(m) ^ 2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}