pub fn mn() {
    let x = 5;

    println!("The first value of x is {}",x);
    // x =7; // immutable so throws an error
    println!("The second value of x is {}",x);
    let x = 8; // shadowing
    println!("The third value of x is {}",x);
    const CONST:u32 = 2;
    println!("The value of const is {}",CONST);


// use let to change mutability and type of variables
let spaces = "    ";
let spaces = spaces.len();
println!("{}",spaces);

}