struct Point {
    x: i32,
    y: i32,
}

// スタックとヒープ
// スタック = 静的なメモリ
// ヒープ = 動的なメモリ
// スタックよりもヒープの方が多く確保できる

// BOXとは
// オブジェクトをヒープ領域に確保できる
// 使い所
// 自作の構造体をヒープ領域に確保したい時。
// 巨大なデータを扱う時、スタックでは賄えない。

pub fn run() {
    let t1 = (3, "bird".to_string()); // i32とstringのタプル。スタック領域に存在する。
    let mut b1 = Box::new(t1); // Boxポインタを作る。ヒープ領域に移動する。
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "bird".to_string()));
}