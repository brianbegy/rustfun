use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's play a game...");
    println!("Guess what number I'm thinking of...");

    println!("Choose easy mode or hard mode (type 'hard' for hard mode.) .");
    let mut mode = String::new();

    io::stdin()
        .read_line(&mut mode)
        .ok()
        .expect("Error reading line...");

    let chosen_mode = get_mode(mode);

    println!("Ok {} mode it is.", chosen_mode);

    let min: u32 = 1;
    let max: u32 = 100;

    let secret_number = rand::thread_rng().gen_range(min..=max);

    loop {
        println!("Ok...guess a number between {} and {}.", min, max);

        let guess: u32 = get_guess();
        println!("You guessed: {}", guess);

        if evaluate_guess(guess.clone(), chosen_mode.clone(), secret_number) {
            break;
        }
    }
}

fn get_mode(input: String) -> String {
    if input == "hard" {
        return "hard".to_string();
    } else {
        return "easy".to_string();
    }
}

fn get_guess() -> u32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .ok()
        .expect("Error reading line...");
    let guess: u32 = guess.trim().parse().expect("Must be a number.");
    return guess;
}

fn evaluate_guess(guess: u32, chosen_mode: String, secret_number: u32) -> bool {
    if chosen_mode == "easy" {
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                return false;
            }
            Ordering::Greater => {
                println!("Too big!");
                return false;
            }
            Ordering::Equal => {
                println!("You win!  The number was {}.", secret_number);
                return true;
            }
        }
    } else {
        if secret_number == guess {
            println!("You win!  The number was {}.", secret_number);
            return true;
        } else {
            println!("Sorry, wrong.");
            return false;
        }
    }
}
