//adding external dependencies
extern crate rand;

// Using std
use std::io;

use std::cmp::Ordering;

//Someethin, something, scope, methods, something ...
use rand::Rng;

fn main() {
    println!("Guess the number, bro!");

    //call thread_rng - calls a particular random generator
    //.gen_range(1, 101) -> call that method (param, param) 
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);
    println!("Input the guess:");

    let mut guess = String::new();

    // later on &mut
    //using io to read input
    io::stdin().read_line(&mut guess)
    //.expect -> error handling
    .expect("Failure to read input!");

    // Set number to unsigned 32 bit int
    // 
    let guess: u32 = guess.trim().parse()
    .expect("Add a number!");

    println!("Your guess is {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Correct!"),
    }
}

