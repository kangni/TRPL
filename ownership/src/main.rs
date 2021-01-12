fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    let s4 = String::from("xixi");
    let s5 = s4;
    println!("{}", s5);

    println!("{} {}", s1, s3);

    //
    let x = String::from("world");
    let (y, len) = calculate_length(x);

    println!("The len of {} is {}", y, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(a_string: String) -> (String, usize) {
    let length = a_string.len();

    (a_string, length)
}