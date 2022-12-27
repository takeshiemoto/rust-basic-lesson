pub fn run() {
    let members = vec!["John", "Paul", "George", "Ringo"];
    render(&members);
    println!("{:?}", members);
}

fn render(members: &Vec<&str>) {
    for i in members {
        println!("{}", i);
    }
}
