fn main() {
    //Ownership is a set of rules that Rust uses to manage memory
    //1.- Each value in Rust has an Owner
    //2.- There can only be one owner at a time
    //3.- When the owner goes out of scope the value will be droppped

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    //
    // THE STRING DATA TYPE
    // Will be useful to ilustrte ownership
    //A string literal:
    let s = "Hola";
    //are inmmutable, and is allocated in the stack
    //The Dara Type String allocates the data on the heap
    let mut s = String::from("Hola");
    // This strings can be mutated
    s.push_str(", Alvaro");

    // MOVE data
    //
    // When you try to assing the same string to another variable the program does not create a
    // copy
    let mut b = s;
    // That sentece is just telling b to point at the same block of memory where s lives
    // So, when the variables goes out of scope it produces a double free error, as they are trying
    // to free the same block of memory.
    // Therefore, once you duplicate a pointer the first one IS NO LONGER VALID
    // This is what Rust call A MOVE
    //
    // CLONE
    //
    // IF we want to create a copy of some string we can use the clone method
    let sc = b.clone();
    // These concepts also aply on function calls
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
