// Approach 1: Option<T>
// These are the components of Option
// enum Option<T>{ // define the generic optioin type
//     Some(T), // repesents a value
//     None, // repesent no value
// }

// Approach 2: Result<T, E>
// these are the components of Result
// enum Result<T, E>{ // define generic Result type
//     Ok(T), // represents a value
//     Err(E), // represents an error
// }

fn divide_option(numerator: f64, denominator: f64) -> Option<f64>{
    if denominator == 0.0 {
        None
    } else {
        Some(numerator/denominator)
    }

}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator/denominator)
    }
}

fn main() {

    let result = divide_option(10.0, 2.0);
    match  result{
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero!"),
    };

    match  divide_result(100.0, 0.0){
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    };




}
