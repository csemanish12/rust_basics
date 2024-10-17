fn main() {
    // loop
    // loop{
    //     println!("hello world!");
    // }

    // returning value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels -> when you use nested loops

    let mut counter = 0;

    'counting_up: loop {
        let mut remaining = 10;        
        loop {
            println!("Remaining is: {remaining}");
            remaining -= 1;

            if remaining == 8 {
                break;
            }
        }
        println!("counter is: {counter}");
        counter += 1;

        if counter == 5 {
            break 'counting_up;
        }
        
    }

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("Number is : {number}");
        number -= 1;
    }

    // for loop -> looping through a collection
    println!("Loop ended");

    let a = [1,2,3,4,5,6];
    for element in a {
        println!("The element is: {element}");
    }

}
