use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // io::stdin().read_line(&mut number).expect("Number must between 1-100");

    // println!("You guessed: {number}");

    loop {
        println!("Please enter a number between 1-100: ");

        // number has to be redeclared each loop iteration or program will crash on parse
        let mut number: String = String::new();
        io::stdin().read_line(&mut number).expect("Number must between 1-100");

        // parsing number from string, assigning to guess
        let guess: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("number value: {number}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


}
