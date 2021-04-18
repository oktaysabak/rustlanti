fn main() {
    println!("hello from main function");
    let my_number: u8 = 10;
    let my_number = my_number + 13;
    let heart_eyed_cat = '😻';

    println!("my new shadowed number is: {}", my_number);
    println!("my cat: {}", heart_eyed_cat);

    another_function();
    another_function_with_parameter(65);
    //let five = five();

    println!("five function result is {}", five());
    println!("five function result is {}", plus_one(five()));

    struct Person {
        name: String,
        age: u8,
        likes_oranges: bool,
    }

    let person = Person {
        name: String::from("Okito"),
        likes_oranges: true,
        age: 25,
    };

    println!("new person is {}", person.name);
    println!("he likes oranges {}", person.likes_oranges);
    println!("his age is {}", person.age);
    /*
    enum WebEvent {
        // An enum may either be unit-like,
        PageLoad,
        PageUnload,
        // An enum can include characters and strings
        KeyPress(char),
        Paste(String),
        // or include tuple structs
        Click { x: i64, y: i64 },
    }
    This enum has four variants with different types:

    PageLoad and PageUnload have no data associated with it at all.
    Keypress includes a single character in it.
    Paste includes a single string.
    Click includes an anonymous struct inside it.
    */
    indexing_tuple();
    indexing_array();

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for item in 0..5 {
        println!("{}", item * 2);
    }
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

fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1;
    println!("indexing tuple function...");

    assert_eq!(
        2, second,
        "This is not the 2nd number in the tuple: {}",
        second
    )
}

fn indexing_array() {
    let characters = ['a', 'b', 'c', 'd', 'e'];
    let letter_d = characters[3];
    println!("indexing array function...");
    assert_eq!(
        'd', letter_d,
        "This is not the character for the letter d: {}",
        letter_d
    )
}
