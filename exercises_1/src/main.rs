fn main() {
    let x: i32 = 5; // immutable variable
    let mut y: i32 = 5; // mutable variable

    println!("x: {}", x);
    println!("y: {}", y);

    y = 10; // reassign value to y
    println!("y: {}", y);

    let z = 5; // type inference
    println!("z: {}", z);

    let a = 5.0; // f64
    let b: f32 = 5.0; // f32
    println!("a: {}", a);
    println!("b: {}", b);

    let c = 'a'; // char
    println!("c: {}", c);

    let d = true; // bool
    println!("d: {}", d);

    // string
    let e = "Hello, world!";
    println!("e: {}", e);

    let f = String::from("Hello, world!"); // heap allocated string
    println!("f: {}", f);

    let f: &str = "Hello, world!"; // string slice
    println!("f: {}", f);
}
