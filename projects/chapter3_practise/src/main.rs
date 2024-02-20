use std::io;

fn main() {
    convert_fahrenheit_to_celcius();
}

fn convert_fahrenheit_to_celcius() {
    //could get input like 50.0C and use last char -> remove it -> then convert float thats left
    
    loop {
        println!("Type F to convert from fahrenheit to celcius.");
        println!("--------------------Or--------------------");
        println!("Type C to convert from celcius to farenheit.");

        let mut temp_type = String::new();

        io::stdin()
            .read_line(&mut temp_type)
            .expect("Couldn't read line");

        let temp_type: char = match temp_type.trim().parse() {
            Ok(character) => character,
            Err(_) => {
                println!("Please enter either F or C");
                continue
            },
        };

        println!("Enter the temperature to convert (if you want to convert 50{temp_type}, enter 50.0");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Couldn't read line");
        
        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature such as 52.0 or 7.0");
                continue;
            }
        };

        let result: f32;
        if temp_type == 'F' {
            result = (temp - 32.0) * 5.0/9.0;
            println!("{}F in C is {}C.", {temp}, {result});
            break;
        } else if temp_type == 'C' {
            result = (temp *(9.0 / 5.0)) + 32.0;
            println!("{}C in F is {}F.", {temp}, {result});
            break;
        } else {
            println!("Please enter either F or C");
        }

    }
}

fn nth_fibonacci_number(x: i32) {

}

fn christmas_carol_lyrics() {

}
