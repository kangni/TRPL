fn main() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let len = calculate_length(&s1);

    let r1 = &s1;
    let r2 = &s1;

    println!("The length of '{}' is {} ", s1, len);

    println!("{} & {}", r1, r2);

    let r3 = &mut s1;
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world.");
}