extern crate rand;

use std::io ;
use rand::Rng;
use std::cmp::Ordering;


fn main(){
    println!("\nWelcome to this number guessing game, Press q to quit ");
    let secret_num: u32 = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("Guess a number: ");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Could not read input");

        let guess = guess.trim();
        if guess.eq_ignore_ascii_case("q") {
            println!("Thanks for playing!");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        }
            ;      

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small\n"),
            Ordering::Greater => println!("Too big\n"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
}
}

