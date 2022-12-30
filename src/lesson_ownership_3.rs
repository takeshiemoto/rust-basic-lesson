// use std::ops::Drop;

#[derive(Debug, Copy, Clone)]
struct Parent(usize, Child, Child);

#[derive(Debug, Copy, Clone)]
struct Child(usize);

// デストラクタを実装している場合
// Copyトレイトが実装できない
// impl Drop for Parent {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self);
//     }
// }
//
// impl Drop for Child {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self);
//     }
// }

pub fn run() {
    let p1 = Parent(1, Child(11), Child(12));
    let p2 = p1;
    println!("{:?}, {:?}", p1, p2);
}
