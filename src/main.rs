use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    let x = 10;
    let y = 20;
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Some random prints x = {}, y = {}, secret num = {}", x,y,secret_number);


    if rand::random() { // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", rand::random::<char>());
    }

    println!("Guess the number game!");

    println!("Input the number");
    
    let _apples = 5;


    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

    


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is too small"),
            Ordering::Greater => println!("Number is too large"),
            Ordering::Equal => {
                println!("Correct, you win");
                break;
            },
        }        
    }
    
    println!("Finished");
}
