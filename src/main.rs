fn main() {
    // s has the type String and it's stored in the heap
    let hello_rust = String::from("hello rust");
    // and doesn't have a Copy trait
    // so using a reference to avoid ownership by println!,
    // even though it takes references

    println!("{}", &hello_rust);

    let numbers: [f32; 5] = [10.1, 20.2, 30.3, 40.4, 50.5];
    // an array of 5 elements of type float32 bit
    
    for number in numbers { // iterate through the numbers arr
        println!("{}", number);
    }

    let tup: (char, f32, u8) = ('a', 2.0, 240);
    // tuples and array are known as compound types, and tuples can be ,
    // destructured, just as in js

    let (a, two_point_zero, two_hundred_and_fourthy) = tup;

    println!("char is {}, f32 is {}, u8 is {}", a, two_point_zero, two_hundred_and_fourthy);
    // you could also get the values of a tuple through indexing
    println!("firts element of tup is {}", tup.0);

    // then we have structs, (to structure data), funny thing is,
    // it looks like js Object

    struct Rectangle {
        width: u32,
        height: u32
    }

    let rect_1 = Rectangle {
        width: 30,
        height: 40
    };

    println!("the width of rect_1 is {}", rect_1.width);
}
