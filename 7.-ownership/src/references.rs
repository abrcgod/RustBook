fn references_borrowing() {
    //Instead of passing the ownership of our data from function to function and,
    //therefore, having to return tuples with the original data and the result of the
    //function if we want to keep using that data; we can use refereces
    let s1 = String::from("hello"); // We create a string

    let len = calculate_length(&s1); // In the function call we use a & symbol to say
                                     // this is a reference to the string

    println!("The length of '{}' is {}.", s1, len); // That way we haven´t pass ownership so the
                                                    // string is still useable in this scope

    // References are inmutable by default, so, if we want a to allow changes to the variable we
    // have tu use mutable references
    change(&mut s);
    // NOTE: Mutables variables can have inmutable references, this is, even if the variable itself
    // is mutable, when yo pass an inmutable reference to that variable you are not allowing any
    // funtion to mutate it
    //

    //  Yout can't have multples mutable references at once
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // THis is an error

    println!("{}, {}", r1, r2);

    // REFERENCING RULES
    //At any given time, you can have either one mutable reference or any number of immutable references.
    //References must always be valid.

    // String slices
    // A slice is a refefence to a portion of a string
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// The function specifies that it recieves a reference, so, it's borrowing the data
fn calculate_length(s: &String) -> usize {
    s.len()
} // Given that s was borrowed it is not dropped at this end of scope

// This function specifies that you should pass a mutable reeference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// This is  a function used to find the first_word in a string
// and returning a slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {}
