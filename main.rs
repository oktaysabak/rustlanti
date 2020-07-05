fn main() {
    println!("hello from main function");

    another_function();
    another_function_with_parameter(65);
    //let five = five();

    println!("five function result is {}", five());
    println!("five function result is {}", plus_one(five()));
}

fn another_function() {
    println!("hello from another function.");
}

fn another_function_with_parameter(x: i32) {

    println!("the value of x is: {}", x);
}

//Functions with Return Values

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}