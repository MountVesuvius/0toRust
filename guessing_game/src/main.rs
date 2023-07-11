use rand::{self, Rng};
use std::io; 

fn main() {
    let mut rng = rand::thread_rng();
    let number: usize = rng.gen_range(1..100);

    loop {
        println!("Guess a number between 1 and 100");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        if n < 1 || n > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        if n < number {
            println!("Too small!");
        } else if n > number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }

    }

}
