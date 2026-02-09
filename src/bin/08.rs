use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let mut total_size = 0;
    
    for line in input.lines() {
        let internal_str = &line[1..line.len() - 1];
        let code_size = internal_str.len() + 2;
        let mut in_memory_string_size = 0;
    
        let mut chars_it = internal_str.chars();
    
        while let Some(c) = chars_it.next() {
            if c == '\\'
                && let Some(next_c) = chars_it.next()
            {
                chars_it = chars_it.dropping(match next_c {
                    // we skip 0 times except for 'x' as we already advanced the iterator
                    // once to check the next character
                    '\\' => 0,
                    '"' => 0,
                    'x' => 2,
                    // input is guaranteed to contain only the cases above
                    _ => unreachable!("Bad input, unknow escape sequence"),
                });
            }
    
            in_memory_string_size += 1;
        }
    
        total_size += code_size - in_memory_string_size;
    }
    
    Some(total_size)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|line| &line[1..line.len() - 1])
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '\\' => 1,
                        '"' => 1,
                        _ => 0,
                    })
                    .sum::<usize>()
                    + 4
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
