fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {}", result);

    let a = [0, 1, 2, 3, 4];
    let mut index = 0;

    while index < 5 {
        println!("value: {}", a[index]);
        index = index + 1;
    }

    let b = [2, 4, 6, 8, 10];

    for value in b.iter() {
        println!("value: {}", value);
    }

    for num in (1..4).rev() {
        println!("{}", num);
    }
}
