pub fn bottles_song(starting_bottles: u128) {
    let mut bottles: u128 = starting_bottles;
    while bottles > 2 {
        println!("{} bottles of beer on the wall,", bottles);
        println!("{} bottles of beer.", bottles);
        println!("Take one down, pass it around,");
        bottles -= 1;
        println!("{} bottles of beer on the wall.", bottles);
        println!();
    }
    if bottles == 2 {
        println!("{} bottle of beer on the wall,", bottles);
        println!("{} bottle of beer.", bottles);
        println!("Take one down, pass it around,");
        bottles -= 1;
        println!("{} bottle of beer on the wall.", bottles);
        println!();
    }
    if bottles == 1 {
        println!("{} bottle of beer on the wall,", bottles);
        println!("{} bottle of beer.", bottles);
        println!("Take one down, pass it around,");
        bottles -= 1;
        println!("No bottles of beer on the wall.");
        println!();
    }
    if bottles == 0 {
        println!("No bottles of beer on the wall,");
        println!("No bottles of beer.");
        println!("Go to the store, buy some more,");
        bottles = starting_bottles;
        println!("{} bottles of beer on the wall.", bottles);
    }
}
