pub fn run() {
    // ルール1
    // 可変参照は何個でも同時に存在できる
    let a = 10;
    let a_ref1 = &a;
    let a_ref2 = &a;

    println!("{}, {}, {}", a, a_ref1, a_ref2); // 10, 10, 10
}
