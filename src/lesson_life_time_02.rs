use crate::lesson_life_time_01::ToyVec;

pub fn run() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budgerigar".to_string());
    let e = v.get(1);
    assert_eq!(e, Some(&"Budgerigar".to_string()))
}
