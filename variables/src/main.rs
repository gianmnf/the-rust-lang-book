fn main() {
    //Mutable variable / Shadowing
    /* let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;

    println!("Subscriber Count: {}", SUBSCRIBER_COUNT);

    // Data Types

    // Integers
    let a = 98_22; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (U8)

    // Above 255 crashes
    let f: u8 = 255;

    // Floating-point numbers
    let f = 2.0;
    let g: f32 = 3.0;

    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let t = false;

    // Character
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    // Compound Types
    let tup = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    //Arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    // Out of bounds
    // let x = error_codes[3];
    // Array with 8 values, all set to 0
    let byte = [0; 8]; */

    /* let sum = my_function(11, 22);
    println!("The sum is: {}", sum); */

    // Control Flow

    /* let number = 5;

    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number); */

    // Loops

    /* let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {}", result); */

    // While Loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // For loop

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}

// Functions - all lowercase, use underscore for spaces
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}
