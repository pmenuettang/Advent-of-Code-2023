use core::cmp;
use std::{fs, io};

/// --- Day 1: Trebuchet?! ---
///
/// Something is wrong with global snow production, and you've been selected to take a look.
/// The Elves have even given you a map; on it, they've used stars to mark the top fifty locations
/// that are likely to be having problems.
///
/// You've been doing this long enough to know that to restore snow operations,
/// you need to check all fifty stars by December 25th.
///
/// Collect stars by solving puzzles.
/// Two puzzles will be made available on each day in the Advent calendar;
/// the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
///
/// You try to ask why they can't just use a weather machine ("not powerful enough")
/// and where they're even sending you ("the sky")
/// and why your map looks mostly blank ("you sure ask a lot of questions")
/// and hang on did you just say the sky ("of course, where do you think snow comes from")
/// when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
///
/// As they're making the final adjustments, they discover that
/// their calibration document (your puzzle input) has been amended by a very young Elf
/// who was apparently just excited to show off her art skills.
/// Consequently, the Elves are having trouble reading the values on the document.
///
/// The newly-improved calibration document consists of lines of text;
/// each line originally contained a specific calibration value that the Elves now need to recover.
/// On each line, the calibration value can be found
/// by combining the first digit and the last digit (in that order)
/// to form a single two-digit number.
///
/// For example:
///
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
///
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the calibration values?
///
/// Returns the sum of all of the calibration values on success
///
/// Returns an `io::Error` on failure
/// ```
pub fn day1_step1() -> Result<u64, io::Error> {
    let input = fs::read_to_string("input/day1.txt")?;
    let sum = input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let first = first_digit(line).unwrap_or_default();
            let last = last_digit(line).unwrap_or_default();

            first * 10 + last
        })
        .sum::<u64>();

    Ok(sum)
}

/// Returns the first digit of the line on success
///
/// Returns `None` if there is no digit
///
/// `line` : the line to check
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use advent_of_code_2023::day1::first_digit;
/// let digit = first_digit("abc4e6gh9jkl".as_bytes());
/// assert_eq!(digit, Some(4));
/// ```
pub fn first_digit(line: &[u8]) -> Option<u64> {
    for c in line.iter() {
        let digit = byte_to_u64(*c);
        if digit.is_some() {
            return digit;
        }
    }

    None
}

/// Returns the last digit of the line on success
///
/// Returns `None` if there is no digit
///
/// `line` : the line to check
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use advent_of_code_2023::day1::last_digit;
/// let digit = last_digit("abc4e6gh9j11l".as_bytes());
/// assert_eq!(digit, Some(1));
/// ```
pub fn last_digit(line: &[u8]) -> Option<u64> {
    for c in line.iter().rev() {
        let digit = byte_to_u64(*c);
        if digit.is_some() {
            return digit;
        }
    }

    None
}

/// Returns the an optional digit matching this char on success, None if this is not a digit
///
/// `c` : the byte of the char to check
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use advent_of_code_2023::day1::byte_to_u64;
/// assert_eq!(byte_to_u64(b'a'), None);
/// assert_eq!(byte_to_u64(b'2'), Some(2));
/// ```
pub fn byte_to_u64(c: u8) -> Option<u64> {
    if c.is_ascii_digit() {
        Some((c - b'0') as u64)
    } else {
        None
    }
}

/// --- Part Two ---
///
/// Your calculation isn't quite right.
/// It looks like some of the digits are actually spelled out with letters:
/// one, two, three, four, five, six, seven, eight, and nine
/// also count as valid "digits".
///
/// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
///
/// two1nine
/// eightwothree
/// abcone2threexyz
/// xtwone3four
/// 4nineeightseven2
/// zoneight234
/// 7pqrstsixteen
///
/// In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
///
/// What is the sum of all of the calibration values?
///
/// Returns the sum of all of the calibration values on success
///
/// Returns an io::Error on failure
///
pub fn day1_step2() -> Result<u64, io::Error> {
    let input = fs::read_to_string("input/day1.txt")?;
    let sum = input
        .lines()
        .map(|l| {
            let line = l.as_bytes();
            let first = first_named_digit(line).unwrap_or_default();
            let last = last_named_digit(line).unwrap_or_default();

            first * 10 + last
        })
        .sum::<u64>();

    Ok(sum)
}

const MAX_NAMED_DIGIT_LEN: usize = 5; // three, seven and eight
const MIN_NAMED_DIGIT_LEN: usize = 3; // one, two and six
const ONE: [u8; 3] = [b'o', b'n', b'e'];
const TWO: [u8; 3] = [b't', b'w', b'o'];
const THREE: [u8; 5] = [b't', b'h', b'r', b'e', b'e'];
const FOUR: [u8; 4] = [b'f', b'o', b'u', b'r'];
const FIVE: [u8; 4] = [b'f', b'i', b'v', b'e'];
const SIX: [u8; 3] = [b's', b'i', b'x'];
const SEVEN: [u8; 5] = [b's', b'e', b'v', b'e', b'n'];
const EIGHT: [u8; 5] = [b'e', b'i', b'g', b'h', b't'];
const NINE: [u8; 4] = [b'n', b'i', b'n', b'e'];
const ALL_DIGITS: [(&[u8], u64); 9] = [
    (ONE.as_slice(), 1),
    (TWO.as_slice(), 2),
    (THREE.as_slice(), 3),
    (FOUR.as_slice(), 4),
    (FIVE.as_slice(), 5),
    (SIX.as_slice(), 6),
    (SEVEN.as_slice(), 7),
    (EIGHT.as_slice(), 8),
    (NINE.as_slice(), 9),
];

/// Returns the first digit of the line on success, 0 if there is no digit
///
/// `line` : the line to check
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use advent_of_code_2023::day1::first_named_digit;
/// let digit = first_named_digit("abc4esixghninejkl".as_bytes());
/// assert_eq!(digit, Some(4));
///
/// let digit = first_named_digit("abcde".as_bytes());
/// assert_eq!(digit, None);
/// ```
pub fn first_named_digit(line: &[u8]) -> Option<u64> {
    let len = line.len();

    match len.cmp(&MIN_NAMED_DIGIT_LEN) {
        cmp::Ordering::Less => {
            let digit = first_digit(line);
            if digit.is_some() {
                return digit;
            }
        }
        cmp::Ordering::Equal => {
            let digit = slice_to_first_u64(line);
            if digit.is_some() {
                return digit;
            }
        }
        cmp::Ordering::Greater => {
            for i in 0..=len - MIN_NAMED_DIGIT_LEN {
                let end = cmp::min(i + MAX_NAMED_DIGIT_LEN, len);
                let slice = &line[i..end];

                let digit = slice_to_first_u64(slice);
                if digit.is_some() {
                    return digit;
                }
            }
        }
    }

    let digit = first_digit(line);
    if digit.is_some() {
        return digit;
    }

    None
}

fn slice_to_first_u64(slice: &[u8]) -> Option<u64> {
    let digit = byte_to_u64(slice[0]);
    if digit.is_some() {
        return digit;
    }

    for (needle, value) in ALL_DIGITS {
        if slice.starts_with(needle) {
            return Some(value);
        }
    }

    None
}

/// Returns the last digit of the line on success, 0 if there is no digit
///
/// `line` : the line to check
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use advent_of_code_2023::day1::last_named_digit;
/// let digit = last_named_digit("abc4e6ghninejelevenl".as_bytes());
/// assert_eq!(digit, Some(9));
///
/// let digit = last_named_digit("abcde".as_bytes());
/// assert_eq!(digit, None);
/// ```
pub fn last_named_digit(line: &[u8]) -> Option<u64> {
    let len = line.len();

    match len.cmp(&MIN_NAMED_DIGIT_LEN) {
        cmp::Ordering::Less => {
            let digit = last_digit(line);
            if digit.is_some() {
                return digit;
            }
        }
        cmp::Ordering::Equal => {
            let digit = slice_to_last_u64(line);
            if digit.is_some() {
                return digit;
            }
        }
        cmp::Ordering::Greater => {
            for i in (MIN_NAMED_DIGIT_LEN..=len).rev() {
                let start = i - cmp::min(i, MAX_NAMED_DIGIT_LEN);
                let slice = &line[start..i];

                let digit = slice_to_last_u64(slice);
                if digit.is_some() {
                    return digit;
                }
            }
        }
    }

    if let Some(digit) = last_digit(line) {
        return Some(digit);
    }

    None
}

fn slice_to_last_u64(slice: &[u8]) -> Option<u64> {
    let last = slice.len() - 1;
    let digit = byte_to_u64(slice[last]);
    if digit.is_some() {
        return digit;
    }

    for (needle, value) in ALL_DIGITS {
        if slice.ends_with(needle) {
            return Some(value);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digit() {
        let digit = first_digit("abc4ef7".as_bytes());
        assert_eq!(digit, Some(4));

        let digit = first_digit("abcdefghijkl".as_bytes());
        assert_eq!(digit, None);
    }

    #[test]
    fn test_last_digit() {
        let digit = last_digit("abc4ef7".as_bytes());
        assert_eq!(digit, Some(7));

        let digit = last_digit("abcdefghijkl".as_bytes());
        assert_eq!(digit, None);
    }

    #[test]
    fn test_byte_to_u64() {
        assert_eq!(byte_to_u64(b'z'), None);
        assert_eq!(byte_to_u64(b'0'), Some(0));
        assert_eq!(byte_to_u64(b'1'), Some(1));
        assert_eq!(byte_to_u64(b'2'), Some(2));
        assert_eq!(byte_to_u64(b'3'), Some(3));
        assert_eq!(byte_to_u64(b'4'), Some(4));
        assert_eq!(byte_to_u64(b'5'), Some(5));
        assert_eq!(byte_to_u64(b'6'), Some(6));
        assert_eq!(byte_to_u64(b'7'), Some(7));
        assert_eq!(byte_to_u64(b'8'), Some(8));
        assert_eq!(byte_to_u64(b'9'), Some(9));
    }

    #[test]
    fn test_first_named_digit() {
        let digit = first_named_digit("abthreede6gh9jkl".as_bytes());
        assert_eq!(digit, Some(3));

        let digit = first_named_digit("aone".as_bytes());
        assert_eq!(digit, Some(1));

        let digit = first_named_digit("onea".as_bytes());
        assert_eq!(digit, Some(1));

        let digit = first_named_digit("81s".as_bytes());
        assert_eq!(digit, Some(8));

        let digit = first_named_digit("ab4".as_bytes());
        assert_eq!(digit, Some(4));

        let digit = first_named_digit("fjbbtgone5".as_bytes());
        assert_eq!(digit, Some(1));

        let digit = first_named_digit("7v".as_bytes());
        assert_eq!(digit, Some(7));

        let digit = first_named_digit("xiv".as_bytes());
        assert_eq!(digit, None);
    }

    #[test]
    fn test_last_named_digit() {
        let digit = last_named_digit("abc4e6gh8jelevenl".as_bytes());
        assert_eq!(digit, Some(8));

        let digit = last_named_digit("twoa".as_bytes());
        assert_eq!(digit, Some(2));

        let digit = last_named_digit("atwo".as_bytes());
        assert_eq!(digit, Some(2));

        let digit = last_named_digit("81s".as_bytes());
        assert_eq!(digit, Some(1));

        let digit = last_named_digit("ab4".as_bytes());
        assert_eq!(digit, Some(4));

        let digit = last_named_digit("abcdef".as_bytes());
        assert_eq!(digit, None);
    }
}
