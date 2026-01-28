use advent_of_code::utils::string_utils::StrExtensionsUtils;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split_lines()
            .map(|str_dimensions| {
                let num_dimensions: Vec<u64> = str_dimensions
                    .split('x')
                    .filter_map(|d| d.parse::<u64>().ok())
                    .collect();
                let [l, w, h]: [u64; 3] = num_dimensions.try_into().unwrap();
                let sides = [l * w, w * h, h * l];
                sides.iter().map(|s| s * 2).sum::<u64>() + sides.iter().min().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split_lines()
            .map(|str_dimensions| {
                let mut num_dimensions: Vec<u64> = str_dimensions
                    .split('x')
                    .filter_map(|d| d.parse::<u64>().ok())
                    .collect();
                num_dimensions.sort();
                num_dimensions.iter().take(2).map(|n| n + n).sum::<u64>()
                    + num_dimensions.iter().product::<u64>()
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
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
