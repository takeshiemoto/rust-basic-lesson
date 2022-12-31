pub fn run() {
    // 配列
    let a1 = ["Hello", "world", "rust"];
    let a2 = a1;
    println!("{:?} {:?}", a1, a2);

    // スライス
    // 連続したメモリ領域に同じ型の要素が並んでいるデータ構造ならどれでも対象。
    // 配列に対して&をつければOK
    let a3 = ['l', 'g', 't', 'm'];
    print_info("&a[..]", &a3[..]);
    print_info("&a[..]", &a3);
    print_info("&a[1..3]", &a3[1..3]);

    // ベクタ版
    // &つければOK
    let v1 = vec!['h', 'e', 'l', 'l', 'o'];
    print_info("&vec[..]", &v1);
    print_info("&vec", &v1);
    print_info("&vec[1..3]", &v1[1..3]);

    // ベクタも配列も型強制により&[T]に変換される
    let char_vec = vec!['a', 'b', 'c'];
    let char_ary = ['a', 'b', 'c'];

    print_char_slice(&char_vec);
    print_char_slice(&char_ary);
}

fn print_char_slice(sl: &[char]) {
    println!("{:?}", sl);
}

fn print_str_slice(sl: &[&str]) {
    println!("{:?}", sl);
}

fn print_info(name: &str, sl: &[char]) {
    println!(
        "{:9} - {}, {:?}, {:?}, {:?}",
        name,
        sl.len(),
        sl.first(),
        sl[1],
        sl.last()
    );
}
