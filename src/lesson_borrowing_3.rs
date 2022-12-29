pub fn run() {
    /* --- 不変参照は何個でも作ることができる --- */
    let str_a = "Hello world";
    let str_a_ref_1 = &str_a;
    let str_a_ref_2 = &str_a;
    let str_a_ref3 = &str_a;

    println!("{}{}{}", str_a_ref_1, str_a_ref_2, str_a_ref3);

    /* --- 不変参照と可変参照は同時に存在することはできない --- */
    // let str_a_ref_4 = &mut str_a; // Err

    /* --- 可変参照はひとつしか存在できない--- */
    let mut str_b = "Hello Rust";
    let str_ref_1 = &mut str_b;
    let str_ref_2 = &mut str_b;
    // println!("{}{}", str_ref_1, str_ref_2); // Err
}
