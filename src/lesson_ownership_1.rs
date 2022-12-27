pub fn run() {
    /* --- 所有権がムーブする ---*/
    // String型はMoveが発生する
    let s1 = String::from("Hello world");
    // s1は所有権を失う
    let s2 = s1;
    // println!("s1={}, s2={}", s1, s2); // Err

    /* --- 所有権がムーブしない ---*/
    // &strはコピートレイトを実装している為Moveは発生しない
    let s3 = "Hello world";
    let s4 = s3;
    println!("s3={}, s4={}", s3, s4);

    /* --- 所有権がムーブしない ---*/
    let s5 = String::from("Ruts");
    let s6 = s5.clone();
    println!("s5={}, s6={}", s5, s6);
}

pub fn run2() {
    /* --- 所有権を奪う関数 --- */
    let s1 = String::from("Rust programming .");
    let result_s1 = print_from_value(s1);
    // println!("{}, {}", s1, result_s1); // Err

    /* --- 所有権を奪わない関数--- */
    let s2 = String::from("Hello Rust!!");
    let result_s2 = print_from_reference(&s2);

    println!("{}, {}", s2, result_s2);
}

fn print_from_value(value: String) -> String {
    value
}

fn print_from_reference(value: &String) -> &String {
    value
}
