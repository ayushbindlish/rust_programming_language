use std::io; // import input/output library from standard library
use rand::Rng; // defines methods that random number generators implement, and this trait must be in scope for us to use those methods. 
use std::cmp::Ordering;

fn main() {
    println!("Guess the number"); // println! is a macro
    
    /*rand::thread_rng function will give us the particular 
    random number generator that we’re going to use: one that 
    is local to the current thread of execution and seeded by 
    the operating system. */
    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("The secret number is: {}", secret_number);

    // creating an infinite loop
  loop {
    println!("Please input your guess");
    let mut guess = String::new(); // mutable using "mut"
    // String::new() --> new is associated function (static method)) of String type
    let _foo = 2; // immutable by default
    
    
    
    io::stdin().read_line(&mut guess) // &mut guess is a mutable reference to guess
    .expect("Failed to read line"); 

    // redeclaring the same variable is known as shadowing
//    let guess:u32=guess.trim().parse().expect("Please type a number!");
    // trim() method eliminates any starting and ending white space
    // parse converts string to number, we specify number format using u32 
    // expect for exception handling when string entered is other than a number and parse cannot convert the string to number

    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You won!");
            break;
        }
    }
  }
}