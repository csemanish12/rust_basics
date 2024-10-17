fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in inner scope: {}", x);
    }

    println!("The value of x is main function: {}", x);

    let x = "hello world";
    println!("The value of x after changing its type: {x}");
}
