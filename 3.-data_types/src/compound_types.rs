// TUPLE
// A tuple is a way to group a set of values, which can be of diffentes types;
fn compound() {
    let tup: (i32, f64, u8) = (1000, 20.40, 7);
    // Once the tuple is created it cannot change it's size
    // To access a single element you can either destructure the tupple:
    let (x, y, z) = tup;
    println!("{x}");
    // Or you can use indexing
    let one_thousand = tup.0;

    // Arrays
    // Arrays are collections of elements whith the same type
    let arr = [1, 2, 3, 4, 5, 6, 7];
    // To explicitly anotate the type of the array you have to:
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Yoy can set an initial value for all the elementes this way:
    let arr = [0; 10];
    // Where you type de initial value, then a semicolon and the size of the array.
    //
    //To access individual elements of an array you have to use indexing:
    let ej = arr[3];
}
