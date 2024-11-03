use rand::Rng;
use std::cmp::Ordering;
use std::error::Error;
use std::io;
use rprompt::prompt_reply;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(message :&str) -> Result<Guess, Box<dyn Error>> {
        let value = prompt_reply(message)?
            .trim()
            .parse()
            .unwrap_or(-1);
        
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Ok(Guess { value })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


pub fn main() -> Result<(), Box<dyn Error>> {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {

        let guess = Guess::new("Please input your guess: ")?;
        
        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break Ok(());
            }
        }
    }
}
