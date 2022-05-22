use std::cmp::Ordering;

fn main()
{
    println!("\nLooping with loop construct:");
    let mut cnt: i32 = 1;
    loop {
        match cnt.cmp(&5){
            Ordering::Less => println!("Iteration {}, next...", cnt),
            Ordering::Equal =>{
                println!("Iteration {}, exiting...", cnt);
                break;
            },
            Ordering::Greater => break
        };
        cnt +=1 ;
    }


    println!("\nLooping with while loop:");
    let mut cnt2: u32 = 5;
    while cnt2 > 0 {
        println!("On iteration {}, continue", cnt2);
        cnt2 -= 1;
    } // pretty clean huh?


    println!("\nLooping through array with for loop:");
    let my_arr = [23, 345, 1295, 93246];

    for elem in my_arr.iter(){
	println!("The element is: {}", elem);
    }

    for elem in my_arr.iter().rev(){
	println!("The reverse element is: {}", elem);
    }

    println!("\nLooping through range:");
    for num in 1..6 { // you can also do this in reverse if you'd like
        println!("Number is: {}", num);
    } // so so much cleaner than while loop
}
