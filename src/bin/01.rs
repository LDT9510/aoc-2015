advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let result = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;

    for (i, c) in input.char_indices() {
        sum += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if sum == -1 {
            return Some(i + 1);
        }
    }

    // cannot happen, or the puzzle is wrong
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(5));
    }
}
