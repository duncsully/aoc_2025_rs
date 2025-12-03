/// Day 1 is always a warm up. Everything should be fairly self-explanatory. The only note
/// to make is that since a full rotation is 100 clicks, we can ignore everything past the
/// tens digit and truncate the value. Whether this was better to do to the string before
/// parsing or to the number after parsing I didn't test.
pub fn solver_01_1(input: &str) -> u32 {
    let mut dial = 50;
    let mut zero_count = 0;
    for rotation in input.lines() {
        let (dir, amount) = rotation.split_at(1);
        let truncated_amount = &amount[amount.len().saturating_sub(2)..];
        let clicks: i32 = truncated_amount.parse().expect("Unable to parse number");
        let multiplier = if dir == "L" { -1 } else { 1 };
        dial += clicks * multiplier;
        if dial > 99 {
            dial -= 100;
        } else if dial < 0 {
            dial += 100;
        }
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

/// Funny, in optimizing the solution to the first problem I ended up requiring more
/// modifications to solve the second. But that's alright. Definitely not as clean
/// since I have to check that we didn't start on 0 in order to count a rotation as
/// passing 0. This could probably be cleaned up with some more thought.
pub fn solver_01_2(input: &str) -> i32 {
    let mut dial = 50;
    let mut zero_count = 0;
    for rotation in input.lines() {
        let (dir, amount) = rotation.split_at(1);
        let clicks: i32 = amount.parse().expect("Unable to parse number");
        zero_count += clicks / 100;
        let clicks = clicks % 100;
        let multiplier = if dir == "L" { -1 } else { 1 };
        let started_on_zero = dial == 0;
        dial += clicks * multiplier;
        if dial > 99 {
            if !started_on_zero {
                zero_count += 1;
            }
            dial -= 100;
        } else if dial < 0 {
            if !started_on_zero {
                zero_count += 1;
            }
            dial += 100;
        } else if dial == 0 && !started_on_zero {
            zero_count += 1;
        }
    }
    zero_count
}

/// Yeah, this one was pretty brute-forcey, which for Rust isn't much of a problem.
/// In hindsight, I could've maybe tried to find the common "prefix" of a range and
/// only examine the remaining range.
pub fn solver_02_1(input: &str) -> u64 {
    let ranges = input.split(',');
    let mut total = 0;
    for range in ranges {
        let mut parts = range.split('-');
        let start: u64 = parts
            .next()
            .expect("Didn't have starting value")
            .parse()
            .expect("Couldn't parse number");
        let end: u64 = parts
            .next()
            .expect("Didn't have ending value")
            .parse()
            .expect("Couldn't parse number");
        for value in start..=end {
            let str_value = value.to_string();
            if str_value.len() % 2 == 1 {
                continue;
            };
            let (first, second) = str_value.split_at(str_value.len() / 2);
            if first == second {
                total += value;
            }
        }
    }
    total
}

/// I feel less bad about this brute force approach. Slightly modified approach to
/// iterate through factors up to n/2 instead of only checking n/2.
pub fn solver_02_2(input: &str) -> u64 {
    let ranges = input.split(',');
    let mut total = 0;
    for range in ranges {
        let mut parts = range.split('-');
        let start: u64 = parts
            .next()
            .expect("Didn't have starting value")
            .parse()
            .expect("Couldn't parse number");
        let end: u64 = parts
            .next()
            .expect("Didn't have ending value")
            .parse()
            .expect("Couldn't parse number");
        for value in start..=end {
            let str_value = value.to_string();
            // Go through factors and see how many matches of a substring there are.
            // e.g. for a number with 6 digits, we can divide substrings into 1, 2,
            // and 3 digits. If we end up with 6, 3, or 2 matches respectively then
            // we know we have a repeating substring and the number is invalid.
            for n in 1..=str_value.len() / 2 {
                if str_value.len() % n == 0
                    && str_value.matches(&str_value[..n]).count() == str_value.len() / n
                {
                    total += value;
                    // We don't need to do any more checks for this number
                    break;
                }
            }
        }
    }
    total
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_01_1_works() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(solver_01_1(input), 3);
    }

    #[test]
    fn solver_01_2_works() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(solver_01_2(input), 6);
    }

    #[test]
    fn solver_02_1_works() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(solver_02_1(input), 1227775554);
    }

    #[test]
    fn solver_02_2_works() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(solver_02_2(input), 4174379265);
    }
}
