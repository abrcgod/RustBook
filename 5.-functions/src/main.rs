fn main() {
    println!("Hello, world!");

    //function call
    another_function();
    //function call with parameters
    parameters(100);

    //STATEMENTS and EXPRESSIONS
    //Functios bodies consists on a series of statements an
    //optionally ends with a expression
    //
    //A STATEMENT is a instruction than perform an action and does not return a value
    //A EXPRESSION are evaluations to some value that is to be returned
    //
    //In the example we use an expression to evaluate the value assign to Y with the let statement
    //Function and macros calls are expression too.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    // Function with return value
    let b = square(10.0);
}

//fn keyword to declare a function
// use snake-case names for functions
fn another_function() {
    println!("Another Function");
}

// Declare parameters inside parenthesis, the types have to be especify
fn parameters(x: i32) {
    println!("The value of X is: {x}");
}

// We have to write the type of the returned value after an arrow
fn square(x: f64) -> f64 {
    //there is no need to use a return keyword, we can just type the last expression
    //without a semicolon.
    x * x
}
