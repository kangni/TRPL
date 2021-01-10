fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    let mut z = 5;
    println!("x: {}", z);

    z = 6;
    println!("x: {}", z);

    let z = 5;

    let z = z + 1;

    let z = z * 2;

    println!("z: {}", z);

    // tuple
    let tup = (1, 2.0, 3);
    let (a, b, _) = tup;
    println!("tup: ({}, {}, {})", a, b, tup.2);

    // list
    let x: [i32; 5] = [0, 1, 2, 3, 4];
    println!("x: {:?}", x);

    let x = [3; 5];
    println!("x: {:?}", x);
}