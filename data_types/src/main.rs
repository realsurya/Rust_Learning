use std::io;

fn main() {
    // Data types:
    // Scalar types represent single vals:
    //     - int
    //     - float
    //     - bool
    //     - char

    
    // Integers:
    //     they come in i (signed) and u (unsigned) variants
    //     can be i or u 8, 16, 32, 64, 128
    println!("\nTesting Integers:");
    let x: i8 = -2; // signed 8-bit integer
    let y: u8 = 2; // unsigned 8-bit integer
    println!("The values of ints x and y are: {}, {}", x, y);

    // Floats!
    // come in f32, 64 variants. defaults to 64.
    println!("\nTesting Floats:");
    let x1 = 2.5; //f64
    let y1: f32 = -3.5; //f32
    println!("The values of floats x1 and y1 are: {}, {}", x1, y1);

    // Booleans
    // can be implict or explicit
    println!("\nTesting Booleans:");
    let x2 = true;
    let y2: bool = false;
    println!("The values of bools x2 and y2 are: {}, {}", x2, y2);

    // Characters
    // can be defined much the same as bool
    println!("\nTesting Characters:");
    let x3 = 'a';
    let y3: char = 'z';
    println!("The values of chars x3 and y3 are: {}, {}", x3, y3);


    // COMPOUND TYPES!
    //     - tuple
    //     - array

    // Tuples
    // basically allow you to assign var groups
    println!("\nTesting Tuples:");
    let x4: (i32, f32, bool) = (100, 3.14, true);
    let (a, b, c) = x4; //unpack
    println!("The tuple x4 unpacked into a, b, c are: {}, {}, {}", a, b, c);
    let y4: (i32, f32, bool) = (256, 1.14, false);
    println!("The tuple y4 unpacked using y4.0, y4.1, y4.2: {}, {}, {}", y4.0, y4.1, y4.2);

    // Arrays
    // can define dynamically:
    //     let x = [1, 2, 3];
    // to create an array with 10 5s:
    //     let x = [5; 10];
    // to explicitly def type, size:
    //     let x: [i32, 4] = [1, 2, 3, 4];
    println!("\nTesting Arrays:");
    let x5 = [1, 2, 3, 4, 5];
    println!("The value of the 0th element of array x4 is: {}", x5[0]);

    println!("Enter index of x5 you would like to query:");
    let mut idx = String::new();
    io::stdin().read_line(&mut idx).expect("Failed to read line");

    let idx: usize = match idx.trim().parse(){
	Ok(num) => num,
	Err(_) => {
    	    println!("Error: Enter an integer please!");
    	    return;
	}
    };

    println!("the value of x5 at index {} is: {}", idx, x5[idx]);
}
