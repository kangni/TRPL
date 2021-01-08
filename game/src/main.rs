use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Start");

    loop {
        println!("input your number: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        let secret_num = rand::thread_rng().gen_range(1, 101);

        // println!("The secret num is: {}", secret_num);

        match guess.cmp(&secret_num) {
            Ordering::Equal => {
                println!("bingo");
                break;
            },
            Ordering::Less => println!("too small\n"),
            Ordering::Greater => println!("too big\n"),
        }
    }

}