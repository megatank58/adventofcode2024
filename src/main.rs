use std::fs::read_to_string;

fn main() {
    let input = &read_to_string("src/input.txt".to_string()).unwrap();

    day1::part1(input);
    day1::part2(input);
}

mod day1 {
    use std::collections::HashMap;

    pub fn part1(input: &str) {
        let mut a1 = vec![];
        let mut a2 = vec![];
        let mut sum = 0;

        for line in input.lines() {
            let numbers = line
                .split_whitespace()
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            a1.push(numbers[0]);
            a2.push(numbers[1]);
        }

        a1.sort();
        a2.sort();

        for (i, v) in a1.iter().enumerate() {
            sum += (v - a2[i]).abs()
        }

        dbg!(sum);
    }

    pub fn part2(input: &str) {
        let mut a = vec![];
        let mut b = HashMap::new();
        let mut sum = 0;

        for line in input.lines() {
            let numbers = line
                .split_whitespace()
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            a.push(numbers[0]);

            b.insert(numbers[1], b.get(&numbers[1]).unwrap_or(&0) + 1);
        }

        for v in a.iter() {
            sum += v * b.get(&v).unwrap_or(&0)
        }

        dbg!(sum);
    }
}
