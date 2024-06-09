use std::{cmp::Ordering, io};
use rand::Rng;
use colored::*;

fn main() {
    
    
    let secret_number = rand::thread_rng().gen_range(0..=100);
    println!("Guess a Number : ");

    loop{

        let mut guess= String::new(); 

        io::stdin().read_line(&mut guess).expect("Failed to real Line");

        let guess: u32 = guess.trim().parse().expect("Please type a Number");

        println!("You guess : {guess}");

        match guess.cmp(&secret_number){
            Ordering::Equal => {
                println!("{}","You Won".green());
                break;
            },
            Ordering::Greater => println!("{}","Guess Lower".red()),
            Ordering::Less => println!("{}","Guess Higher".red()),
        }
    }

}
