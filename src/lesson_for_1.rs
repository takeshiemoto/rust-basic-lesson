pub fn run() {
    // 右側を含まない範囲演算子
    for n in 1..10 {
        println!("{}", n)
    }

    // 右側を含む範囲演算子
    for n in 1..=10 {
        println!("{}", n)
    }
}
