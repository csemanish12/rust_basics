fn main() {
    // arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // second parameter is size of array

    println!("Number Array: {:?}", numbers);

    // let mix = [1,2,"apple"];   -> cannot have mixed data types inside an array
    // println!("mix is: {:?}", mix)

    let fruits: [&str;3] = ["apple", "banana", "orange"];
    println!("Fruits array: {:?}", fruits);
    println!("Fruits array first element: {}", fruits[0]);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    // here "Alice" is a string slice and not string. so it needs to be converted to string.
    println!("Human tuple: {:?}", human);

    let my_mix_tuple = ("kratos", 23, true, [1,2,3,4,5]);
    println!("my mix tuple: {:?}", my_mix_tuple);


    // Slices:
    // [1,2,3,4,5] -> meaning these are stored in contiguos memory location
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number slice: {:?}", number_slices);

    let animal_slices: &[&str] = &["dog", "cat", "lion"];
    println!("Animal slice: {:?}", animal_slices);

    let book_slices: &[String] = &["harry potter".to_string(), "LOR".to_string()];
    println!("book slice: {:?}", book_slices);


    // Strings vs String Slices
    let mut stone_cold: String  = String::from("Hell, ");
    stone_cold.push_str("Yeah");
    println!("Stone cold sats: {}", stone_cold);

    // &str -> string slice
    let my_string: String = String::from("Hello, World");
    let slice: &str = &my_string[0..5];
    println!("Slice value: {}", slice)


}
