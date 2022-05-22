use std::io;
use String;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    let mut guess = String::new();
    let secret = rand::thread_rng().gen_range(1 , 101);
        
    println!("Welcome to the world's best guessing game!");
    println!("Input a guess:");

    io::stdin().read_line(&mut guess)
	.expect("Failed to read line!");

    let guess: u32 = guess.trim()
        .parse()
        .expect("Please enter a number!");

    println!("The guess was: {}", guess);
    println!("the secret was: {}", secret);

    match guess.cmp(&secret)
    {
        Ordering::Less => println!("Your guess was too small!"),
        Ordering::Equal => println!("Looks like you win! Congrats!"),
        Ordering::Greater => println!("Your guess was too big!"),
    }
}
