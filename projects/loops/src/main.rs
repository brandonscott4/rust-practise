//Rust has three kinds of loops: loop, while, and for.
fn main() {
    //loop {
    //    println!("again!");
    //}

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //------------------------------------------------

    //If you have loops within loops, break and continue apply to the innermost loop at that point. 
    //You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. 
    //Loop labels must begin with a single quote.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //------------------------------------------------

    //while loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //while loop to loop through an array
    //NOTE: This is slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //------------------------------------------------

    //for loop
    //faster and safer (no need for the runtime check and eliminates the risk of out of bounds error)
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    //for loop is most commonly used loop
    //and can be used with range to match functionality of lift off code (most rustaceans would...)
    //use Range (provided by the standard library) which generates all numbers in sequence starting from one number and ending before another number.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
