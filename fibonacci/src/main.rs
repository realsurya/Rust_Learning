use std::io;

fn main()
{
    let mut num_seq = String::new();

    println!("Enter number of fibonacci sequence entries you want to see:");
    
    io::stdin().read_line(&mut num_seq).expect("Failed to get input");
    let num_seq: u32 = match num_seq.trim().parse(){
	Ok(num) => num,
	Err(_) => {
    		println!("Please enter an integer.");
    		return;
	}
    };

    fib(num_seq);
}

fn fib(num_seq: u32) {
   let mut before: i128 = 0;
   let mut after: i128 = 1;

   println!("\n\n{}", before);
   println!("{}", after);

   for _curr_index in 0..(num_seq - 2) {
       let now = before + after;
       println!("{}", now);
       before = after;
       after = now;
   }

}
