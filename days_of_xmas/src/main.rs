fn main()
{
    let gifts = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "three French hens"),
        ("fourth", "four calling birds"),
        ("fifth", "five gold rings"),
        ("sixth", "six geese a-laying"),
        ("seventh", "seven swans a-swimming"),
        ("eight", "eight maids a-milking"),
        ("ninth", "nine ladies dancing"),
        ("tenth", "ten lords a-leaping"),
        ("eleventh", "eleven pipers piping"),
        ("twelth", "twelve drummers drumming")];
    
    for day in 0..gifts.len() {
    	let curr_day = gifts[day].0;
	println!("\nOn the {} day of Christmas, my true love sent to me", curr_day);
	for day_rev in (0..day + 1).rev(){
	   let curr_gift = gifts[day_rev].1;

	   if (day != 0) && (day_rev == 0) {
    	       println!("and {}.", curr_gift)
	   }else if day_rev == 0 {
               println!("{}.", curr_gift);
	   }else{
               println!("{},", curr_gift)
	   }
	}
    }
}
