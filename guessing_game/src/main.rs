use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {

    println!("Guess The number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop{
        println!("Enter your input guess:");
        let mut guess=String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // converting the string into int, and handling errors. if any char entered apart from number 
        // it accept as Error and continue to next iteration
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Greater => println!("number is too big"),
            Ordering::Less => println!("number is too small"),
            Ordering::Equal => {println!("You guessed Correct, You win");
                                break;}
        }

    }
}
