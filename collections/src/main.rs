fn main() {
    let v: Vec<i32> = Vec::new(); // creates new vector
    // macro for doing above
    let v2: Vec<i32> = vec![1,2,3];

    let mut v3: Vec<i32> = Vec::new();
    // pushing elements in the vector
    v3.push(5);
    v3.push(10);
    v3.push(6);
    println!("v3 is: {:?}", v3);

    // reading elements of vector by indexing
    let third: &i32 = &v3[2];
    println!("Third element:{}", third);

    // reading element by using get method
    let second: Option<&i32> = v3.get(1);
    match second {
        Some(second) => println!("The second element from get method is:{}", second),
        None => println!("There is no second element"),
    }

    // collection type - UTF8
    let s = "whatever".to_string();

    let s2 = String::from("whatever");
    
    let mut s3 = String::from("foo");
    s3.push_str("bar"); // for string slice
    s3.push('!'); // single quote for character
    println!("The value of s3: {}", s3);

    // concat
    let s4 = s2 + &s3; // previous occurance of s2 is moved here as s4 took ownership of s2
    // plus doesnot work with more than one owned variable

    println!("s4 is: {}", s4);

    // formatting string
    let value1 = String::from("hello");
    let value2 = String::from("world");
    let full = format!("{value1} {value2}");
    println!("full message is: {full}");
}
