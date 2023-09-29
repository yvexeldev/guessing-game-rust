use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {
    println!("Guess the number! 🧠");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("👀 The secret number is: {}", secret_number);

    println!("Please input your guess 👇");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line ❌");

        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please input valid number! ❌ 👇".red());
                continue;
            }
        };
        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Less than 🌚".yellow()),
            Ordering::Greater => println!("{}", "Greater then 🌚".yellow()),
            Ordering::Equal => {
                println!("{}", "You won! 🥳".green());
                break;
            },
        }
    }
}
