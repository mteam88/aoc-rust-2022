pub fn part_one(input: &str) -> Option<u32> {
    // split by newlines
    let elves: Vec<&str> = input.split("\n\n").collect();
    // split each elf by newlines
    let elves: Vec<Vec<&str>> = elves.iter().map(|elf| elf.split("\n").collect()).collect();
    // sum each elf
    let elves: Vec<u32> = elves.iter().map(|elf| elf.iter().map(|line| line.parse::<u32>().unwrap_or_default()).sum()).collect();
    // get largest elf
    let largest_elf = elves.iter().max().unwrap();
    Some(*largest_elf)
}

pub fn part_two(input: &str) -> Option<u32> {
    // split by newlines
    let elves: Vec<&str> = input.split("\n\n").collect();
    // split each elf by newlines
    let elves: Vec<Vec<&str>> = elves.iter().map(|elf| elf.split("\n").collect()).collect();
    // sum each elf
    let elves: Vec<u32> = elves.iter().map(|elf| elf.iter().map(|line| line.parse::<u32>().unwrap_or_default()).sum()).collect();
    // get top 3 elves
    let mut top_elves = elves.clone();
    top_elves.sort();
    top_elves.reverse();
    top_elves.truncate(3);
    Some(top_elves.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
