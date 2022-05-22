use std::io;

fn main()
{

    let mut my_num: String = String::new();

    println!("\nTesting if else statements:");
    println!("Please enter a number:");
    io::stdin().read_line(&mut my_num).expect("Failed to read line");

    let my_num: i32 = match my_num.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Please input an integer");
	    return;
        }
    }; // gather user input as we have done before

    if my_num < 10
    {
        println!("Your number is {}, which is less than 10", my_num);
    }
    else if my_num > 10
    {
        println!("Your number is {}, which is greater than 10", my_num);
    }
    else
    {
        println!("Your number is {}, which is equal to 10", my_num);
    }


    // Using if statements to dictate let expressions!

    let condition: bool = true;
    let my_num2 = if condition{
        1
    }else{
        0
    };// you can use the if statement to "return" desired value.
      // much like functions, if you use a semicolon, it is no
      // longer a return.

    // since blocks eval to last statement, do not mismatch types.
    // For example, this will **not** work:
    // 
    // let my_num2 = if condition{
    //     1
    // }else{
    //     "nope"
    // }

    
    println!("\nUsing if statements to assign value to let:");
    println!("The condition is {} therefore my_num2 is: {}", condition, my_num2);
}
