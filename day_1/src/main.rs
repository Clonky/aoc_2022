use std::fs;

const SRC: &str = "input.txt";

fn main() {
    let contents: String = fs::read_to_string(SRC).expect("Unable to read file.");
    let lines: Vec<&str> = contents.lines().collect();
    let mut counter = 0;
    let mut summed_calories = vec![];
    for line in lines {
        if line.is_empty() {
            summed_calories.push(counter);
            counter = 0;
        } else {
            counter += line.parse::<i32>().expect("Could not parse line into i32.");
        }
    }
    let max = find_max_and_idx(&summed_calories);
    println!("idx: {:?} val: {:?}", max.0, max.1);
    let mut sum_of_three = 0;
    for _ in 0..3 {
        let imax = find_max_and_idx(&summed_calories);
        sum_of_three += imax.1;
        summed_calories.remove(imax.0);
    }
    println!("{}", sum_of_three)
}

fn find_max_and_idx(list: &[i32]) -> (usize, i32) {
    let mut max_idx = 0;
    let mut max = 0;
    for ical in list.iter().enumerate() {
        if *ical.1 > max {
            max = *ical.1;
            max_idx = ical.0;
        }
    }
    (max_idx, max)
}
