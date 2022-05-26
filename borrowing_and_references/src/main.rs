fn main() {
    println!("Recall with heap data such as str, passing as a param gives ownership. We can change that with references & borrowing.");

    let s1 = String::from("This");
    println!("\nString s1 houses value: '{}'", s1);
    let l1 = str_len(&s1);
    println!("String s1 is length {}. We can still access it because of borrowing: {}", l1, s1);
            // The string s1 cannot be modified by str_len function.

    println!("\nTesting mutable borrowing:");
    let mut s2 = String::from("This is");
    println!("String s2 is currently: '{}', now calling mod function with mut borrow.", s2);
    str_fun(&mut s2);
    println!("S2 is now: '{}'", s2); //remember there can only be
                                     //ONE mutable borrow per var


    println!("\nDo not mix immutable borrows and mutable borrows!");
    let s3 = String::from("Good or bad?");
    let _s4 = &s3;
    let _s5 = &s3;
    // let s6 = &s3; DO NOT DO THIS!
    // Rust also protects from dangling references.
}

fn str_len(instr: &String) -> usize{
    instr.len() // we are passed the reference for the input string
                // therefore don't take ownership!
                // s1 will therefore will not be dropped due to movement.
                // however, instr will be dropped, which is not an issue for us.
}

fn str_fun(instr: &mut String){
    instr.push_str(" now different!"); // we are passed a mutable borrow
                                       // therefore we can change the contents
                                       // of the reference
}


