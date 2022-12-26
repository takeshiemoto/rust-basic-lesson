fn main() {
    let starting_lineup = vec!["John", "Paul", "George", "Ringo"];

    let s1 = &starting_lineup[..2]; // "John", "Paul"
    let s2 = &starting_lineup[2..]; // "George", "Ringo"
    let s3 = &starting_lineup[2..3]; // "George", "Ringo"
    let s4 = &starting_lineup[..]; // "John", "Paul", "George", "Ringo"

    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);
    println!("{:?}", s4);

    // for loopは所有権を奪う
    // for i in starting_lineup {
    //     println!("{}", i);
    // }

    // &で回す
    for i in &starting_lineup {
        println!("{}", i);
    }

    // .iterで回す
    for i in starting_lineup.iter() {
        println!("{}", i);
    }
}
