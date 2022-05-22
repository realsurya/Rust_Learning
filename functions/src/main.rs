fn main() {

    println!("\nTesting other function:");
    // make call to other function
    say_hello();

    println!("\nCalling other function with parameters:");
    let x = "REEEE".to_string();
    say_something(x);

    println!("\nTesting out function scopes:");
    func_scope();

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


