use std::fs::read_to_string;

fn main() {
    let input = &read_to_string("src/input.txt".to_string()).unwrap();

    day2::part1(input);
    day2::part2(input);
}

mod day2 {
    pub fn part1(input: &str) {
        let mut sum = 0;

        for line in input.lines() {
            let values = line
                .split_whitespace()
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let gradient = values[0] < values[1];
            let mut flag = 0;

            for i in 0..values.len() - 1 {
                if !condition(values[i], values[i + 1], gradient) {
                    flag = 1;
                    break;
                }
            }

            if flag == 0 {
                sum += 1;
            }
        }

        dbg!(sum);
    }

    pub fn part2(input: &str) {
        let mut sum = 0;

        for line in input.lines() {
            let values = line
                .split_whitespace()
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let mut positive_gradients = 0;
            let mut negative_gradients = 0;

            for i in 0..values.len() - 1 {
                if values[i] == values[i + 1] {
                    continue;
                } else if values[i] < values[i + 1] {
                    positive_gradients += 1;
                } else {
                    negative_gradients += 1;
                }
            }

            let gradient = positive_gradients > negative_gradients;

            let mut flag = 0;
            let mut times_mutated = 0;
            let mut skip = false;

            for i in 0..values.len() - 1 {
                if skip {
                    skip = false;
                    continue;
                }
                if !condition(values[i], values[i + 1], gradient) {
                    if i == 0 {
                        if condition(values[1], values[2], gradient) {
                            times_mutated += 1;
                        } else if condition(values[0], values[2], gradient) {
                            times_mutated += 1;
                            skip = true;
                        } else {
                            flag = 1;
                            break;
                        }
                    } else if i + 1 == values.len() - 1 {
                        if times_mutated > 0 {
                            flag = 1;
                        }
                    } else if condition(values[i], values[i + 2], gradient) && times_mutated == 0 {
                        times_mutated += 1;
                        skip = true;
                    } else if condition(values[i - 1], values[i + 1], gradient)
                        && times_mutated == 0
                    {
                        times_mutated += 1;
                    } else {
                        flag = 1;
                        break;
                    }
                }
            }

            if flag == 0 {
                sum += 1;
            }
        }

        dbg!(sum);
    }

    fn condition(a: i64, b: i64, gradient: bool) -> bool {
        (a < b) == gradient && [1, 2, 3].contains(&(a - b).abs())
    }
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
