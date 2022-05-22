fn main() {

    println!("\nTesting other function:");
    // make call to other function
    say_hello();

    println!("\nCalling other function with parameters:");
    let x = "REEEE".to_string();
    say_something(x);

    println!("\nTesting out function scopes:");
    func_scope();

    println!("\nTesting function return:");
    println!("The function ret_ten() returns value: {}", ret_ten());

    let my_num: i32 = 5;
    println!("The number {} doubled is: {}", my_num, double_input(my_num));

}

fn say_hello()
{
    println!("This print statement is from another function");
}

fn say_something(given: String)
{
    println!("The given parameter is: {}", given);
}

fn func_scope()
{
    let x = 10;
    
       // unlike in the C lang, x = y = 4 will not work!
      // this is because assignment will not return the
     // value being assigned. So this will *NOT* work:
    //     let x = (let y = 4);  OR let x = y = 4;
   //     instead to def new scope use:

    let y = {
	let x = 20;
	x+1
    };

    println!("The value of x and y are: {}, {}", x, y);
}

fn ret_ten() -> i32
{
    10 // note this literally returns this value, kinda like matlab
      // most functions can return the last statement implicitly,
     // though an early return can be invoked using `return` like C
    //
   // note adding semicolon to this statement will make it an
  // expression, and that vill not be returned
}

fn double_input(inp: i32) -> i32
{
    inp * 2 // can use return in*2.
           // note adding semicolon
          // voids return
}
