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

// simulated_expensive_calculationを強制的に呼び出している。
// この関数をクロージャーを利用してリファクタリングする。
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!(
            // 次に、{}回腹筋をしてください！
            "Next, do {} situps!",
            expensive_result
        );
    } else if random_number == 3 {
        // 今日は休憩してください！水分補給を忘れずに！
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            // 今日は、{}分間走ってください！
            "Today, run for {} minutes!",
            expensive_result
        );
    }
}
