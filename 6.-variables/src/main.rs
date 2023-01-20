// This code won't compile
//
fn main() {
    // MUTABILITY
    //
    // Variables are mutable by default
    //
    // let x = 5;
    // println!("The value of x is: {x}");
    // Given that x was not declare as mutable this line give us 
    // a compile error:
    // x = 6;
    // println!("The value of x is: {x}");
    // To solve the problem we have to declare x as a mutable Variable

    let mut x = 5;
    println!("The value of x is {x}");
    x = 6; // x is mutable, so this is OK 
    println!("The value of x is {x}");

    // CONSTANST
    // Constants are also mutable, but its value is evaluated at compile time 
    
    const  THREE_OURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const THREE_OURS_IN_SECONDS is: {THREE_OURS_IN_SECONDS}");

    // SHADOWING
    // We can reuse a variable name by SHADOWING that Variable
    // To do this we use the let keyword;

    let var = "Hello";
    println!("Var is: {var}");
    let var = 1998;
    println!("Var is: {var}");

    // In the example we use the same name to store a string and then a number
    // But once we shadow the variable the string is no longer availabre to use

    println!("{var}");
}

