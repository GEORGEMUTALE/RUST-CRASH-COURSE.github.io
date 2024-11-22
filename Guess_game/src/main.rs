

// Getting the user input, check whether it is the right form
// from the standard library, we access the input/output library
// use std::io;

// fn main() {
//     println!("Guess the number!");

//     println!("Please input your guess.");
// // Because we shall have different inputs we have to make our variable mutable since in rust by default variables are immutable
// // ::new() is an associated function 
//     let mut guess = String::new(); 

// // & is used to copy data, references are also immutable by default
//     io::stdin()
//     .read_line(&mut guess)
//     .expect("Failed to read line");

//     println!("You guessed: {}", guess);
// }
// Here u can find the set of items obtained from the std library
// https://doc.rust-lang.org/std/prelude/index.html

// MAKING THE GAME MORE FUN
// We shall use the random number generator trait from the rand crae
// who knows how to add a crate and updating it.
// rust crates are obtained from crates.io
// use std::io; 
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {secret_number}");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }

// ADVANCED GAME
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut trials = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
// .trim() removes whitespaces .parse() function that converts a string to given value type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        trials += 1; 

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! in {trials} trials");
                break;
            }
        } 
    }
}
// go implement the if and elseif 