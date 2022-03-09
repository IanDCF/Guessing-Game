use std::io;  // Bring I/O library into scope to obtain user input and then print the result as output
use std::cmp::Ordering;
use rand::Rng; // the Rng trait defines methods that random number generators implement

fn main() {
    println!("Guess a number between 1 - 100");

    let secret_number = rand::thread_rng().gen_range(1..101); //function gives us the particular rand num generator 
                                           // ^^^ method takes a range expression as argument start(inclusive)..end(exclusive)

    // println!("The answer-key is: {}", secret_number);
  loop {
    println!("Enter your guess:");

    let mut guess = String::new();  //create (let) a mutable (mut) variable (guess) to store an input
                                    // String::new() is a function that returns new, empty instnce of a String type
    io::stdin()                     // stdin function from the io module allows us to handle user input
        .read_line(&mut guess)      //calls the read_line method on the standard input handle to get the input from user
                                    //& indicates that the argument is a reference and it must also be mutable
        .expect("Failed to read line");// io::Result has an expect method 
                                       // if [Err] expect crashes program and displays string in argument 
                                       // if [Ok] returns user input?
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        

        println!("Your guess: {}", guess); // prints string with user guess substituting placeholder : {} 
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The answer-key is greater than your guess."),
            Ordering::Greater => println!("The answer-key is lower than your guess."),
            Ordering::Equal => {
                println!("You are CORRECT!");
                break;
                }
            }
        }
    }

