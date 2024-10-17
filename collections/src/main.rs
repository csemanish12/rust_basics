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

}
