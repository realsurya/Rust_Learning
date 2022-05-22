use std::io;
use String;
use rand::Rng;
use std::cmp::Ordering;

fn main()
{
    
    let secret = rand::thread_rng().gen_range(1 , 101);
        
    println!("Welcome to the world's best guessing game!");
    loop
    {
    
    	println!("\nInput a guess:");
	let mut guess = String::new();

    	io::stdin().read_line(&mut guess)
		.expect("Failed to read line!");

    	let guess: u32 = match guess.trim().parse(){
		Ok(num) => num,
		Err(_) => continue
    	};

    	println!("The guess was: {}", guess);
    	println!("the secret was: {}", secret);

    	match guess.cmp(&secret)
    	{
        	Ordering::Less => println!("\nYour guess was too small!"),
       		Ordering::Equal => {
			println!("\nLooks like you win! Congrats!");
			break;
       		},
        	Ordering::Greater => println!("\nYour guess was too big!"),
    	}
    }
}
