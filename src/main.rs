use std::fs::read_to_string;

fn main() {
    let input = &read_to_string("src/input.txt".to_string()).unwrap();

    day7::part1(input);
}

mod day7 {
    use std::collections::HashMap;

    use std::vec::IntoIter;

    use itertools::{Itertools, Permutations, Unique};

    pub fn part1(input: &str) {
        let lines = input.lines();
        let mut sum = 0;

        let mut permutation_cache: HashMap<usize, Unique<Permutations<IntoIter<i32>>>> =
            HashMap::new();

        for line in lines {
            let mut line = line.split(":");
            let result = line.next().unwrap().parse::<i64>().unwrap();

            let values = line
                .next()
                .unwrap()
                .trim()
                .split(" ")
                .map(|f| f.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            let mut selections = vec![];

            for i in 0..values.len() - 1 {
                let mut selection = vec![];
                for _ in 0..i {
                    selection.push(0);
                }
                for _ in i..values.len() - 1 {
                    selection.push(1);
                }
                selections.push(selection);
            }

            let selections_length = selections.len();

            'main: for selection in selections {
                let selection_length = selection.len();

                let permutation = if permutation_cache.contains_key(&selections_length) {
                    permutation_cache.get(&selections_length).unwrap().clone()
                } else {
                    selection
                        .into_iter()
                        .permutations(selection_length)
                        .unique()
                };

                permutation_cache.insert(selections_length, permutation.clone());

                for perm in permutation {
                    let mut r = values[0];
                    let mut c = 1;

                    for i in perm {
                        if i == 0 {
                            r += values[c]
                        } else {
                            r *= values[c]
                        }
                        c += 1;
                    }

                    if r == result {
                        sum += r;
                        break 'main;
                    }
                }
            }
        }

        dbg!(sum);
    }
}

mod day6 {
    use std::collections::HashSet;

    pub fn part1(input: &str) {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut visited_positions = vec![];

        let (mut current_position, mut heading) = get_initial_data(&lines);

        visited_positions.push(current_position);

        loop {
            let r = next_position(current_position, heading, &lines);

            if r.0.is_none() {
                break;
            }

            current_position = r.0.unwrap();
            visited_positions.push(current_position);
            heading = r.1;
        }

        let result = visited_positions
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .len();

        dbg!(result);
    }

    pub fn part2(input: &str) {
        let lines = input.lines().collect::<Vec<&str>>();

        let mut object_positions: Vec<(usize, usize)> = vec![];

        let (mut current_position, mut heading) = get_initial_data(&lines);

        loop {
            let r = next_position(current_position, heading, &lines);

            if r.0.is_none() {
                break;
            }

            current_position = r.0.unwrap();
            heading = r.1;

            let initial_data = get_initial_data(&lines);
            let mut temp_heading = initial_data.1;
            let mut temp_position = initial_data.0;
            let mut temp_positions = vec![];

            let mut flag = false;

            let mut temp_lines = lines.clone();

            if current_position.1 - 1 + [0, 1, 2, 1][heading] >= temp_lines.len() {
                continue;
            }

            let mut temp_line =
                temp_lines[current_position.1 - 1 + [0, 1, 2, 1][heading]].to_string();

            if current_position.0 - 1 + [1, 2, 2, 0][heading] >= temp_line.len() {
                continue;
            }

            let mut temp_chars = temp_line.chars().collect::<Vec<char>>();
            temp_chars[current_position.0 - 1 + [1, 2, 1, 0][heading]] = '#';

            temp_line = temp_chars.iter().collect::<String>();

            temp_lines[current_position.1 - 1 + [0, 1, 2, 1][heading]] = &temp_line;

            loop {
                let r = next_position(temp_position, temp_heading, &temp_lines);

                if r.0.is_none() {
                    break;
                }

                temp_position = r.0.unwrap();
                temp_heading = r.1;

                if temp_positions.contains(&(temp_position, temp_heading)) {
                    flag = true;
                    break;
                }

                temp_positions.push((temp_position, temp_heading));
            }

            if flag {
                object_positions.push((
                    current_position.0 + 1 - [1, 0, 1, 2][heading],
                    current_position.1 + 1 - [2, 1, 0, 1][heading],
                ));
            }
        }

        let result = object_positions
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
            .len();

        dbg!(result);
    }

    fn get_initial_data(lines: &Vec<&str>) -> ((usize, usize), usize) {
        let mut pos = (0, 0);
        let mut heading = 0;

        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if ['<', '>', '^', 'v'].contains(&char) {
                    pos = (j, i);

                    heading = ['^', '>', 'v', '<']
                        .iter()
                        .position(|f| *f == char)
                        .unwrap();
                }
            }
        }

        (pos, heading)
    }

    fn next_position(
        mut current_position: (usize, usize),
        mut heading: usize,
        lines: &Vec<&str>,
    ) -> (Option<(usize, usize)>, usize) {
        let result;
        let max_height = lines.len() - 1;
        let max_width = lines[0].chars().collect::<Vec<char>>().len() - 1;

        match heading {
            0 | 2 => {
                if (current_position.1 == max_height && heading == 2)
                    || (current_position.1 == 0 && heading == 0)
                {
                    result = None;
                } else if lines[current_position.1 - 1 + heading]
                    .chars()
                    .nth(current_position.0)
                    .unwrap()
                    == '#'
                {
                    heading += 1;
                    result = Some(current_position);
                } else {
                    current_position.1 = current_position.1 - 1 + heading;
                    result = Some(current_position);
                }
            }
            1 | 3 => {
                if (current_position.0 == max_width && heading == 1)
                    || (current_position.0 == 0 && heading == 3)
                {
                    result = None;
                } else if lines[current_position.1]
                    .chars()
                    .nth(current_position.0 + 2 - heading)
                    .unwrap()
                    == '#'
                {
                    heading = if heading == 1 { 2 } else { 0 };
                    result = Some(current_position);
                } else {
                    current_position.0 = current_position.0 + 2 - heading;
                    result = Some(current_position);
                }
            }
            _ => unreachable!(),
        }

        (result, heading)
    }
}

mod day5 {
    use std::collections::HashMap;

    pub fn part1(input: &str) {
        let mut lines = input.lines();
        let mut sum = 0;
        let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();

        loop {
            let line = lines.next().unwrap();

            if line == "" {
                break;
            }

            let rule = line.split("|").collect::<Vec<&str>>();
            let key = rule[1];
            let mut value = if rules.contains_key(key) {
                rules.get(key).unwrap().to_vec()
            } else {
                vec![]
            };

            value.push(rule[0]);

            rules.insert(key, value.to_vec());
        }

        for line in lines {
            let mut elements = vec![];

            let numbers = line.split(",").collect::<Vec<&str>>();
            let mut flag = true;

            for number in numbers.iter() {
                elements.push(number);

                let rule = if rules.contains_key(number) {
                    rules.get(number).unwrap().to_vec()
                } else {
                    vec![]
                };

                for n in rule {
                    if numbers.contains(&n) && !elements.contains(&&n) {
                        flag = false;
                    }
                }
            }

            if flag {
                sum += numbers[(numbers.len() - 1) / 2].parse::<u32>().unwrap();
            }
        }

        dbg!(sum);
    }

    pub fn part2(input: &str) {
        let mut lines = input.lines();
        let mut sum = 0;
        let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();

        loop {
            let line = lines.next().unwrap();

            if line == "" {
                break;
            }

            let rule = line.split("|").collect::<Vec<&str>>();
            let key = rule[1];
            let mut value = if rules.contains_key(key) {
                rules.get(key).unwrap().to_vec()
            } else {
                vec![]
            };

            value.push(rule[0]);

            rules.insert(key, value.to_vec());
        }

        let mut unsorted_lists = vec![];

        for line in lines {
            let mut elements = vec![];

            let numbers = line.split(",").collect::<Vec<&str>>();
            let mut flag = true;

            for number in numbers.iter() {
                elements.push(number);

                let rule = if rules.contains_key(number) {
                    rules.get(number).unwrap().to_vec()
                } else {
                    vec![]
                };

                for n in rule {
                    if numbers.contains(&n) && !elements.contains(&&n) {
                        flag = false;
                    }
                }
            }

            if !flag {
                unsorted_lists.push(numbers);
            }
        }

        for list in unsorted_lists {
            let mut elements = vec![];
            let mut numbers = list;

            let mut is_sorted = false;

            while !is_sorted {
                let mut flag = true;
                for number in numbers.iter() {
                    let rule = if rules.contains_key(number) {
                        rules.get(number).unwrap().to_vec()
                    } else {
                        vec![]
                    };

                    for n in rule {
                        if numbers.contains(&n) && !elements.contains(&n) {
                            elements.push(n);
                            flag = false;
                        }
                    }

                    if !elements.contains(&number) {
                        elements.push(*number);
                    }
                }

                if flag == true {
                    is_sorted = true;
                } else {
                    numbers = elements.clone();
                    elements.clear();
                }
            }
            sum += numbers[(numbers.len() - 1) / 2].parse::<u32>().unwrap();
        }
        dbg!(sum);
    }
}

mod day4 {
    pub fn part1(input: &str) {
        let lines = input.lines();

        let mut horizontal = 0;
        let mut token = String::new();

        for line in lines {
            for char in line.chars() {
                token.push(char);

                if token == "XMAS" || token == "SAMX" {
                    horizontal += 1;
                }

                if token.len() == 4 {
                    token.remove(0);
                }
            }

            token.clear()
        }

        let lines = input.lines().collect::<Vec<&str>>();
        let mut vertical = 0;

        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == 'X' && lines.get(i + 3).is_some() {
                    if lines[i + 1].chars().collect::<Vec<char>>()[j] == 'M'
                        && lines[i + 2].chars().collect::<Vec<char>>()[j] == 'A'
                        && lines[i + 3].chars().collect::<Vec<char>>()[j] == 'S'
                    {
                        vertical += 1;
                    }
                }

                if char == 'S' && lines.get(i + 3).is_some() {
                    if lines[i + 1].chars().collect::<Vec<char>>()[j] == 'A'
                        && lines[i + 2].chars().collect::<Vec<char>>()[j] == 'M'
                        && lines[i + 3].chars().collect::<Vec<char>>()[j] == 'X'
                    {
                        vertical += 1;
                    }
                }
            }
        }

        let lines = input.lines().collect::<Vec<&str>>();
        let mut diagonal = 0;

        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == 'X' {
                    if lines.get(i + 3).is_some()
                        && line.chars().collect::<Vec<char>>().get(j + 3).is_some()
                    {
                        if lines[i + 1].chars().collect::<Vec<char>>()[j + 1] == 'M'
                            && lines[i + 2].chars().collect::<Vec<char>>()[j + 2] == 'A'
                            && lines[i + 3].chars().collect::<Vec<char>>()[j + 3] == 'S'
                        {
                            diagonal += 1;
                        }
                    }

                    if lines.get(i + 3).is_some() && j >= 3 {
                        if lines[i + 1].chars().collect::<Vec<char>>()[j - 1] == 'M'
                            && lines[i + 2].chars().collect::<Vec<char>>()[j - 2] == 'A'
                            && lines[i + 3].chars().collect::<Vec<char>>()[j - 3] == 'S'
                        {
                            diagonal += 1;
                        }
                    }
                }

                if char == 'S' {
                    if lines.get(i + 3).is_some()
                        && line.chars().collect::<Vec<char>>().get(j + 3).is_some()
                    {
                        if lines[i + 1].chars().collect::<Vec<char>>()[j + 1] == 'A'
                            && lines[i + 2].chars().collect::<Vec<char>>()[j + 2] == 'M'
                            && lines[i + 3].chars().collect::<Vec<char>>()[j + 3] == 'X'
                        {
                            diagonal += 1;
                        }
                    }

                    if lines.get(i + 3).is_some() && j >= 3 {
                        if lines[i + 1].chars().collect::<Vec<char>>()[j - 1] == 'A'
                            && lines[i + 2].chars().collect::<Vec<char>>()[j - 2] == 'M'
                            && lines[i + 3].chars().collect::<Vec<char>>()[j - 3] == 'X'
                        {
                            diagonal += 1;
                        }
                    }
                }
            }
        }

        dbg!(vertical + horizontal + diagonal);
    }

    pub fn part2(input: &str) {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut sum = 0;

        for (i, line) in lines.iter().enumerate() {
            for (j, char) in line.chars().enumerate() {
                if char == 'A'
                    && i != 0
                    && i != lines.len() - 1
                    && j != 0
                    && j != line.chars().collect::<Vec<char>>().len() - 1
                {
                    if (lines[i - 1].chars().collect::<Vec<char>>()[j - 1] == 'M'
                        && lines[i + 1].chars().collect::<Vec<char>>()[j + 1] == 'S'
                        && lines[i - 1].chars().collect::<Vec<char>>()[j + 1] == 'M'
                        && lines[i + 1].chars().collect::<Vec<char>>()[j - 1] == 'S')
                        || (lines[i - 1].chars().collect::<Vec<char>>()[j - 1] == 'M'
                            && lines[i + 1].chars().collect::<Vec<char>>()[j + 1] == 'S'
                            && lines[i - 1].chars().collect::<Vec<char>>()[j + 1] == 'S'
                            && lines[i + 1].chars().collect::<Vec<char>>()[j - 1] == 'M')
                        || (lines[i - 1].chars().collect::<Vec<char>>()[j - 1] == 'S'
                            && lines[i + 1].chars().collect::<Vec<char>>()[j + 1] == 'M'
                            && lines[i - 1].chars().collect::<Vec<char>>()[j + 1] == 'M'
                            && lines[i + 1].chars().collect::<Vec<char>>()[j - 1] == 'S')
                        || (lines[i - 1].chars().collect::<Vec<char>>()[j - 1] == 'S'
                            && lines[i + 1].chars().collect::<Vec<char>>()[j + 1] == 'M'
                            && lines[i - 1].chars().collect::<Vec<char>>()[j + 1] == 'S'
                            && lines[i + 1].chars().collect::<Vec<char>>()[j - 1] == 'M')
                    {
                        sum += 1;
                    }
                }
            }
        }

        dbg!(sum);
    }
}

mod day3 {
    pub fn part1(input: &str) {
        let mut lex = input.chars().peekable();

        let mut sum = 0;
        let mut token = String::new();

        loop {
            if lex.peek().is_none() {
                break;
            }

            let c = lex.next().unwrap();

            if ['m', 'u', 'l'].contains(&c) {
                token.push(c);
            } else {
                match (c, token.as_str()) {
                    ('(', "mul") => {
                        let mut a = String::new();
                        let mut b = String::new();

                        if lex.peek().is_none() {
                            break;
                        }

                        let c = *lex.peek().unwrap();

                        if c.is_numeric() {
                            a.push(c);
                            lex.next().unwrap();
                            loop {
                                if lex.peek().is_none() {
                                    break;
                                }

                                let c = *lex.peek().unwrap();

                                if c.is_numeric() {
                                    a.push(c);
                                    lex.next().unwrap();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            token.clear();
                            continue;
                        }

                        if lex.peek().is_none() {
                            break;
                        }

                        let c = *lex.peek().unwrap();

                        if c != ',' {
                            token.clear();
                            continue;
                        }

                        lex.next().unwrap();

                        if lex.peek().is_none() {
                            break;
                        }

                        let c = *lex.peek().unwrap();

                        if c.is_numeric() {
                            b.push(c);
                            lex.next().unwrap();
                            loop {
                                if lex.peek().is_none() {
                                    break;
                                }

                                let c = *lex.peek().unwrap();

                                if c.is_numeric() {
                                    b.push(c);
                                    lex.next().unwrap();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            token.clear();
                            continue;
                        }

                        let c = *lex.peek().unwrap();

                        if c != ')' {
                            token.clear();
                            continue;
                        }

                        sum += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
                        token.clear();
                    }
                    _ => token.clear(),
                }
            }
        }

        dbg!(sum);
    }

    pub fn part2(input: &str) {
        let mut lex = input.chars().peekable();

        let mut sum = 0;
        let mut token = String::new();
        let mut flag = true;

        loop {
            if lex.peek().is_none() {
                break;
            }

            let c = lex.next().unwrap();

            if ['m', 'u', 'l', 'd', 'o', 'n', 't', '\''].contains(&c) {
                let is_correct_follow = match (c, token.chars().last().unwrap_or('_')) {
                    ('m', '_')
                    | ('u', 'm')
                    | ('l', 'u')
                    | ('d', '_')
                    | ('o', 'd')
                    | ('n', 'o')
                    | ('\'', 'n')
                    | ('t', '\'') => true,
                    _ => false,
                };

                if !is_correct_follow {
                    token.clear();
                } else {
                    token.push(c);
                }
            } else {
                match (c, token.as_str()) {
                    ('(', "mul") => {
                        let mut a = String::new();
                        let mut b = String::new();

                        if lex.peek().is_none() {
                            break;
                        }

                        let c = *lex.peek().unwrap();

                        if c.is_numeric() {
                            a.push(c);
                            lex.next().unwrap();
                            loop {
                                if lex.peek().is_none() {
                                    break;
                                }

                                let c = *lex.peek().unwrap();

                                if c.is_numeric() {
                                    a.push(c);
                                    lex.next().unwrap();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            token.clear();
                            continue;
                        }

                        if lex.peek().is_none() {
                            break;
                        }

                        let c = *lex.peek().unwrap();

                        if c != ',' {
                            token.clear();
                            continue;
                        }

                        lex.next().unwrap();

                        if lex.peek().is_none() {
                            break;
                        }

                        let c = *lex.peek().unwrap();

                        if c.is_numeric() {
                            b.push(c);
                            lex.next().unwrap();
                            loop {
                                if lex.peek().is_none() {
                                    break;
                                }

                                let c = *lex.peek().unwrap();

                                if c.is_numeric() {
                                    b.push(c);
                                    lex.next().unwrap();
                                } else {
                                    break;
                                }
                            }
                        } else {
                            token.clear();
                            continue;
                        }

                        let c = *lex.peek().unwrap();

                        if c != ')' {
                            token.clear();
                            continue;
                        }

                        if flag {
                            sum += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
                        }

                        token.clear();
                    }
                    ('(', "do") => {
                        flag = true;
                        token.clear();
                    }
                    ('(', "don't") => {
                        flag = false;
                        token.clear();
                    }
                    _ => token.clear(),
                }
            }
        }

        dbg!(sum);
    }
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
