pub fn run() {
    let max = 10;
    let mut x = 0;

    while x <= max {
        let mut y = 0;
        while y <= max {
            if x + y == 10 && (2 * x + 4 * y == 32) {
                println!("鶴={}, 亀={}", x, y);
                y = max + 1;
                x = max + 1;
            } else {
                y += 1
            }
        }
        x += 1;
    }
}
