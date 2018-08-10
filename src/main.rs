extern crate rand;
use std::cmp::Ordering;
use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess_buffer = String::new();
        stdin().read_line(&mut guess_buffer)
            .expect("Failed to read line");
        let guess: u32 = guess_buffer.trim().parse()
            .expect("Please type a number!");

        println!("You guessed: {}", guess);
        if guess.cmp(&secret_number) == Ordering::Equal {
            println!("Correct");
        }
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