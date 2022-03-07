use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_number() {
    println!("Guess the number!");

    let mut secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim().eq_ignore_ascii_case("quit") {
            println!("bye!");
            break;
        }

        if guess.trim().eq_ignore_ascii_case("reset") {
            println!("previous answer is {}", secret_number);
            secret_number = rand::thread_rng().gen_range(1..101);
            println!("and now we reset it.");
            continue;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} not a number", guess);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
