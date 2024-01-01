pub fn part_one() -> i32 {
    let input = include_str!("../input.txt");
    let mut total = 0;
    
    for line in input.lines() {
        // Copilot did most of this...
        // I do appreciate that it's idiomatic Rust.
        // I would not have considered the fold() method.
        let x = line.split("x").collect::<Vec<&str>>();
        let l = x[0].parse::<i32>().unwrap();
        let w = x[1].parse::<i32>().unwrap();
        let h = x[2].parse::<i32>().unwrap();

        let sides = vec![l*w, w*h, h*l];

        let paper_needed: i32 = sides.iter().fold(0, |acc, x| acc + 2*x) + sides.iter().min().unwrap();
        total += paper_needed;
    }

    total
}

pub fn part_two() -> i32 {
    let mut sum = 0;


    sum
}