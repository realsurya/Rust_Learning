fn main() {
    println!("Testing mutability...");

    // let x = 10;
    // x = 20;
    // This will not work since X is an immutable
    // variable by default, and cannot be changed
    // after assignment. Add modifier mut to allow
    // value of var to be changed afterward.


    let mut x: i32 = 10;
    println!("X is: {}", x);

    x = 20;
    println!("X has now changed to: {}", x);


    // constants are only immutable and can be declared
    // in global scope too... type annotation is
    // strictly required for constants


    const TEST_CONST: u32 = 100_000;
    println!("\nTesting constants...");
    println!("The value of constant is: {}", TEST_CONST);


    // Shadowing: like in C, shadowing is basically declaring
    // a new variable that "shadows" a prior variable.
    // This ia different from mut variable as mut is essentially
    // a reassignment, if that makes sense.


    let y = 30;
    println!("\nTesting shadowing...");
    println!("Y is: {}", y);

    let y = y + 1;
    println!("Adding 1 to y thru shadowing: {}", y);


    // Shadowing allows us to mutate variable types.
    // If a variable was Mutable, we cannot change
    // its type. For example, this will not work:
    // let mut myStr = "hello".to_string();
    // myStr = myStr.len();

    println!("\nTesting type mutation...");
    let my_str = "Testing".to_string();
    println!("myStr is currently string with val: {}", my_str);

    let my_str = my_str.len();
    println!("myStr is now int containing lentgh of myStr: {}", my_str);
}
