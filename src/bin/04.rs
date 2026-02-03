use md5::{Digest, Md5};
use std::fmt::Write;

advent_of_code::solution!(4);

fn process(clean_input: &str, byte_to_compare: u8) -> Option<u64> {
    let mut hasher = Md5::new();
    let mut result = 0;
    let mut hash_input = String::with_capacity(100);

    for i in 0..u32::MAX {
        write!(&mut hash_input, "{}{}", clean_input, i).unwrap();

        hasher.update(&hash_input);

        let hash = hasher.finalize_reset();

        if hash[0] == 0 && hash[1] == 0 && hash[2] <= byte_to_compare {
            result = i as u64;
            break;
        }

        hash_input.clear();
    }

    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    process(input.trim_ascii(), 0x0F)
}

pub fn part_two(input: &str) -> Option<u64> {
    process(input.trim_ascii(), 0x00)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609_043));

        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(1_048_970));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6_742_839));
    }
}
