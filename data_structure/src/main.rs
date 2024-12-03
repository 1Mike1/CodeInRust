fn main() {
    // data stuctures
    // arrays
    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", arr); // print the arrays in debug mode

    // arrays are fixed size
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);

    // access elements
    let first = arr[0];
    println!("{}", first);

    // access all elements using length
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    // access all elements using iter | More Rustic way
    for i in arr.iter() {
        println!("{}", i);
    }

    // length of the array
    let len = arr.len();
    println!("{}", len);

    // slices
    let slice = &arr[1..3];
    println!("{:?}", slice);

    // vectors
    let vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);

    // vectors are resizable
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);

    // tuples
    let tup = (1, 2, 3);
    println!("{:?}", tup);

    // destructuring
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    // access elements
    let first = tup.0;
    println!("{}", first);
}
