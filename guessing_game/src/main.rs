use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut guess_counter: u32 = 0;

    loop {
        let mut guess = String::new();
        println!("Input a number:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        // Note: We use "shadowing" to reuse the previous guess variable
        let guess: u32 = match guess.trim()      //  trim() // Removes '\n', '\r' and white-spaces at the beginning and the end of the string
                                    .parse() {   // parse() //Converts a string to another type
                                        Ok(num) => num,
                                        Err(_) => {
                                            println!("!!! Please enter a number"); 
                                            continue;  // can't return an u32 form user input, continue loop instead
                                        },
                                    };

        println!("You guessed: {}", guess);
        guess_counter += 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("To big"),
            Ordering::Equal => {
                println!("You won!");
                println!("The number was guessed in {} guesses", guess_counter);
                break;
            }
        }

    }
    println!("The random number was {secret_number}");
}
