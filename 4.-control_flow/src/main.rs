fn main() {
    // If EXPRESSIONS allow you to branch your code following some condition
    let condition = true;
    if condition {
        let x = 50;
    } else {
        let y = 10;
    }
    // You can have multiple conditions with else if
    let number = 10;
    if number == 0 {
        println!("Number is 0");
    } else if number > 0 {
        println!("Number is positive");
    } else {
        println!("Number is negative");
    }
    // Using IF in an Let statement
    let var = if condition {5} else {6};

    //Repetitions with LOOPS
    //Infinite loop
    loop {
        println!("REPITO");
    }
    // Return values from loops
    let mut counter = 0;
    let var = loop {
        counter += 1;
        if counter == 10 {
            // We can use the break statement like a Return statement
            break counter * 10;
        }
    };

    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // This only stops the current loop     
                break;
            }
            if count == 2 {
                // Now this like will stop the sup loop and not this one
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); 

    //Conditional loop with while
    let mut number = 10;
    while number > 0 {
        println!("Still positive");
        number -= 1;
    }

    // Looping collection with for
    let a = [1, 2, 3 ,4 ,5, 6];
    for element in a {
        println!("{element}");
    }
}

