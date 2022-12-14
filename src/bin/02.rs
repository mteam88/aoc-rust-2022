use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    // define closure to calculate score for one line
    let score = |line: &str| {
        let chars: Vec<&str> = line.split_ascii_whitespace().collect();
        match (chars[0].chars().next().unwrap(), chars[1].chars().next().unwrap()) {
            ('A', 'X') => 4, // 1 + 3
            ('A', 'Y') => 8, // 2 + 6
            ('A', 'Z') => 3, // 3 + 0
            ('B', 'X') => 1, // 1 + 0
            ('B', 'Y') => 5, // 2 + 3
            ('B', 'Z') => 9, // 3 + 6
            ('C', 'X') => 7, // 1 + 6
            ('C', 'Y') => 2, // 2 + 0
            ('C', 'Z') => 6, // 3 + 3
            _ => unreachable!(),
        }
    };
    // calulate score for each line and sum up
    let mut sum = 0;
    while let Some(line) = lines.next() {
        sum += score(line);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    // define closure to calculate score for one line
    let score = |line: &str| {
        let chars: Vec<&str> = line.split_ascii_whitespace().collect();
        match (chars[0].chars().next().unwrap(), chars[1].chars().next().unwrap()) {
            ('A', 'X') => 4, // 1 + 3
            ('A', 'Y') => 8, // 2 + 6
            ('A', 'Z') => 3, // 3 + 0
            ('B', 'X') => 1, // 1 + 0
            ('B', 'Y') => 5, // 2 + 3
            ('B', 'Z') => 9, // 3 + 6
            ('C', 'X') => 7, // 1 + 6
            ('C', 'Y') => 2, // 2 + 0
            ('C', 'Z') => 6, // 3 + 3
            _ => unreachable!(),
        }
    };
    // define another closure to format one line
    let fmt_line = |line: &str| {
        let chars: Vec<&str> = line.split_ascii_whitespace().collect();
        let mut map = HashMap::new();
        map.insert(("A", "X"), ("A", "Z")); // lose rock play scissors
        map.insert(("A", "Y"), ("A", "X")); // draw rock play rock
        map.insert(("A", "Z"), ("A", "Y")); // win rock play paper
        map.insert(("B", "X"), ("B", "Z")); // lose paper play scissors
        map.insert(("B", "Y"), ("B", "Y")); // draw paper play paper
        map.insert(("B", "Z"), ("B", "X")); // win paper play rock
        map.insert(("C", "X"), ("C", "Y")); // lose scissors play paper
        map.insert(("C", "Y"), ("C", "Z")); // draw scissors play scissors
        map.insert(("C", "Z"), ("C", "X")); // win scissors play rock
        
        let mapres = map.get(&(chars[0], chars[1])).unwrap();
        format!("{} {}", mapres.0, mapres.1)
    };
    // calulate score for each line and sum up
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let line = fmt_line(line);
        sum += score(line.as_str());
    }
    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
