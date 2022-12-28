use std::thread;
use std::time::Duration;

pub fn run() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 遅い処理
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// simulated_expensive_calculationを強制的に呼び出している。
// この関数をクロージャーを利用してリファクタリングする。
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // クロージャーには型注釈が無い
    // 1回目に呼び出した型で推論する。
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else if random_number == 3 {
        // 今日は休憩してください！水分補給を忘れずに！
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            // 今日は、{}分間走ってください！
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}
