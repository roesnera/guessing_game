use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Please enter a number between 1-100: ");
    let mut number: String = String::new();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    io::stdin().read_line(&mut number).expect("Number must between 1-100");

    println!("You guessed: {number}");

    let guess: i32 = number.trim().parse().expect("Must be a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }

    
}
