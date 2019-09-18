extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use ::iterators_and_speed::*;

fn main() {
    println!("Guess the number!");
//    let hello = String::from("hello world");
//    let foo = first_word(&hello);
//    let foo2 = first_word(&hello);
//    hello.clear();

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

//        let foo = &guess[0..1];

        debug_print(&guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main2() {
    let rect1 = Rectangle::new(30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

