pub fn run() {
    // 可変
    let mut a = 10;
    // 参照
    let a_ref1 = &a;
    // 一つ目の可変参照
    let a_mut_ref1 = &mut a;
    // 二つ目の可変参照。この時点で一つ目の可変参照は破棄されている。
    let a_mut_ref2 = &mut a;

    *a_mut_ref2 = 20;
    println!("{}", a);
}

pub fn run_error_1() {
    let mut a = 10;
    let a_ref1 = &a;
    // 一つ目の可変参照。
    let a_mut_ref1 = &mut a;
    // 二つ目の可変参照。この時点で一つ目の可変参照は存在しない。
    let a_mut_ref2 = &mut a;

    // 一つ目の可変参照は存在しないのでエラーになる
    // *a_mut_ref1 = 20;
    println!("{}", a);
}

pub fn run_error_2() {
    // 可変と不変が同時に存在するエラー

    // 可変オブジェクト
    let mut a = 10;
    // 不変参照を作る。
    let a_ref1 = &a;
    // 可変参照を作る
    let a_mut_ref1 = &mut a;
    // 二つ目の可変参照を作る。この時点でa_ref1とa_mut_ref1は存在しない。

    // 不変参照と可変参照は同時に存在できない。
    // つまり、a_ref1の不変参照、a_mut_ref1の可変参照にアクセスできてはいけない。
    let a_mut_ref2 = &mut a;

    // エラー
    // println!("{}", a_ref1));
    // エラー
    // println!("{}", a_mut_ref1));
    // 実行時に利用しているのはa_mut_ref2のみ
    println!("{}", a_mut_ref2);
}
