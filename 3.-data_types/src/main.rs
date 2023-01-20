fn main() {
    //INTEGERS
    // 8bits
    let var: i8;
    let var: u8;
    //16 bits
    let var: i16;
    let var: u16;
    //32 bits
    let var: i32;
    let var: u32;
    //64 bits
    let var: i64;
    let var: u64;
    //Match arch
    let var: isize;
    let var: usize;
    // u means unsigned
    
    // LITERALS
    // Decimal:
    let var = 98_200_300;//Can use underscore to improve readability.
    //Hex
    let var = 0xff;
    //Octal
    let var = 0o77;
    // Bin
    let var = 0b1111_0000;
    //Byte (u8 only)
    let var =  b'A';

    //Floating types
    //Simple
    let var: f32;
    //Double;
    let var: f64; // DEFAULT

    //Numeric Operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    // Characters are UTF-8 encoded, so you can store all kinds of modern Characters
    

}
