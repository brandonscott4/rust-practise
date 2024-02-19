const LUCKY_NUM: u32 = 6 + 1;
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("A lucky number is {LUCKY_NUM}");

    shadowing();
}

//the use of let each time (besides the initial) overshadows the previous variable
fn shadowing() {
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    //scope is over and inner shadowing ends, so y returns to being 6

    println!("The value of y is: {y}");


    //when shadowing you can use different types
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The amount of spaces is {spaces}");

    //shadowing works well here because if we tried to use mut we would get a compile time error (we’re not allowed to mutate a variable’s type)
    //let mut spaces = "   ";
    //spaces = spaces.len();
}
