fn main() {
    // Remember:
    // - Each var has an owner, and only 1 owner at a time.
    // - The variable is dropped (freed) at the end of the scope.
    // Recall stack vs heap!

    // S is not in scope here
    let s = "hello"; // S is in scope now
    println!("String s is in scope here: '{}'... but it will be dropped by the end of main.",s);

    // When a string is defined as a string literal it is not mutatable.
    // therefore this will not work:
    // s.push_str("reeeeee");
    //
    // Therefore, define string using String type:
    let mut s2 = String::from("hello");
    println!("String s2 is currently: {}",s2);

    s2.push_str(", world!");
    println!("After appending to string s2 : {}", s2);

    //{ begin scope
    //    x = String::from("testing");
    //} x is now dropped at the end of the scope


    println!("\n\nTesting how String variables interact");
    let s3 = String::from("Hello");
    let s4 = s3; // now s4 refers to teh pointer location of s3.
                 // this is because creating a deep copy is slow in runtime.
                 //
                 // To avoid double free error (since there is only 1 copy of heap data)
                 // Rust considers s3 to be moved to s4. Therefore s3 can NO longer
                 // be accessed.
                 //
                 // Rust will never create deep copies of pointer-based types.
                 // Hoewever, strs can be cloned with
    let s5 = s4.clone();
    println!("s3 has beem moved to s4 ({}). Hoewever, we can still .clone to make deep copy to s5: {}", s4,s5);


   println!("\nTesting how integers interact:");
   let x = 5;
   let y = x; // since int type is of known size,
              // it resides in the stack rather
              // than heap like a string.
              // therefore, y is a copy of x on the stack
              // rather than a pointer reference.
    println!("Int x = {}, and y = x. Y is a copy of x ({}) in this case since size is known. Stack only data can be copied!", x, y);

}
