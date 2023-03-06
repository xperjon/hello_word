use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("The guess-the-number GAME!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please guess a number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read line");


        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            },
        }
    }
}
