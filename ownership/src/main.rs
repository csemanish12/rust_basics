// // example: Each value in Rust has a variable that's its owner

// fn main() {
//     let s1 = String::from("RUST");
//     // s1 is the owner of the value "RUST"

//     let len = calculate_length(&s1); // reference to that string. It borrowed the value by this reference
//     println!("Length of '{}' is: {}", s1, len)
// }




// // Example: There can be only one owner at a time
// fn main(){
//     let s1 = String::from("RUST");
//     let s2 = s1;  // ownership is transferred to s2. Therefore s1 no longer holds the value
    
//     // println!("{}", s1);  // will result in compilation error
//     println!("{}", s2);
// }


// example
fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is: {}", s1, len)

}
// s1 goes out of the scope and its value will be dropper
// fn print_lost(s: &String) {
//     println!("{}", &s1);
// }
// & for reference 
fn calculate_length(s: &String) -> usize {
    s.len()

}