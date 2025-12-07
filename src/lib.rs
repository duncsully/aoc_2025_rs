use std::cmp::{max, min};

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

/// I'm not sure how clever this approach is or not. Basically you know
/// you want to find the highest number you can as far to the left as
/// possible, and then from that index you find the next highest number.
/// It's pretty straight forward from there.
pub fn solver_03_1(input: &str) -> u32 {
    let mut total = 0;
    for line in input.lines() {
        let mut first = 0;
        let mut second = 0;
        let mut index = line.len() - 2;
        for n in (1..=9).rev() {
            if let Some(i) = line.find(&n.to_string())
                && i != line.len() - 1
            {
                index = i;
                first = n;
                break;
            }
        }
        for n in (1..=9).rev() {
            if line[index + 1..].find(&n.to_string()).is_some() {
                second = n;
                break;
            }
        }
        total += first * 10 + second
    }
    total
}

/// It's amusing when the second part requires a more generic solution, thus
/// resulting in a more elegant one. Essentially, you always need to "leave room"
/// to find more numbers, so you have a sliding window in which to search for the
/// largest number you can. After that, you move the window bounded on the left by
/// the index of the previously found value and on the right still by having enough
/// values left to find. If the amount you need is the remaining string, just take it.
pub fn solver_03_2(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let mut on_values = String::new();
        let mut next_start = 0;
        for reserve in (1..=12).rev() {
            // The remaining string is the number of values we still need, just
            // take the rest
            if reserve + next_start - 1 == line.len() {
                on_values.push_str(&line[next_start..]);
                break;
            }
            for n in (1..=9).rev() {
                if let Some(i) = line[next_start..=line.len() - reserve].find(&n.to_string()) {
                    next_start += i + 1;
                    on_values.push_str(&n.to_string());
                    break;
                }
            }
        }
        total += on_values.parse::<u64>().expect("Couldn't parse number");
    }
    total
}

/// I'd like to say it was a straight forward mapping and iterating over a 2D vector
/// but it ended up looking a little less elegant than I'd like.
pub fn solver_04_1(input: &str) -> u32 {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|char| (char == '@') as u8).collect())
        .collect();
    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 0 {
                continue;
            }
            let mut neighbors = 0;
            for y_offset in y.saturating_sub(1)..=min(y + 1, grid.len() - 1) {
                for x_offset in x.saturating_sub(1)..=min(x + 1, row.len() - 1) {
                    if y != y_offset || x != x_offset {
                        neighbors += grid[y_offset][x_offset];
                    }
                }
            }
            if neighbors < 4 {
                total += 1
            };
        }
    }
    total
}

/// So the general concept is simple: keep doing what we did before, but set the removable
/// cells to 0 between iterations. I got a little hung up on Rust's borrow checker, so a
/// more optimal version would edit in place which could potentially remove more items per
/// pass, but ah well this works well enough.
pub fn solver_04_2(input: &str) -> u32 {
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|char| (char == '@') as u8).collect())
        .collect();
    let mut total = 0;
    let mut done = false;
    while !done {
        done = true;
        let mut to_remove = Vec::<(usize, usize)>::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 0 {
                    continue;
                }
                let mut neighbors = 0;
                for y_offset in y.saturating_sub(1)..=min(y + 1, grid.len() - 1) {
                    for x_offset in x.saturating_sub(1)..=min(x + 1, row.len() - 1) {
                        if y != y_offset || x != x_offset {
                            neighbors += grid[y_offset][x_offset];
                        }
                    }
                }
                if neighbors < 4 {
                    total += 1;
                    to_remove.push((y, x));
                    done = false;
                };
            }
        }
        to_remove.iter().for_each(|(y, x)| grid[*y][*x] = 0);
    }

    total
}

/// Another brute force approach. I thought about trying to consolidate
/// the ranges and doing other clever stuff but...just bump to u64s and
/// Rust cranks through it no problem. :shrug:
pub fn solver_05_1(input: &str) -> u64 {
    // Good ol' windows complicating things...
    let double_newline = if input.contains("\r") {
        "\r\n\r\n"
    } else {
        "\n\n"
    };
    let mut parts = input.split(double_newline);
    let ranges: Vec<(u64, u64)> = parts
        .next()
        .expect("Input not properly split")
        .lines()
        .map(|line| {
            let mut range = line.split('-');
            (
                range
                    .next()
                    .expect("No min value")
                    .parse()
                    .expect("Couldn't parse min"),
                range
                    .next()
                    .expect("No max value")
                    .parse()
                    .expect("Couldn't parse max"),
            )
        })
        .collect();
    parts
        .next()
        .expect("Input not properly split")
        .lines()
        .fold(0, |total, id| {
            let id = id.parse::<u64>().expect("Couldn't parse ID");
            if ranges.iter().any(|(min, max)| *min <= id && id <= *max) {
                total + 1
            } else {
                total
            }
        })
}

/// ...I guess I'm gonna consolidate the ranges. I mean, I was curious and lazy enough
/// to see if I could just throw them all in a hash set. It took too long. Not gonna
/// lie, this was the first one where I just kept running into case after case and it
/// took me a while to finally nail this one down, but I'm happy with the result. It's
/// not the prettiest solution, but it's still somewhat elegant IMO.
pub fn solver_05_2(input: &str) -> u64 {
    // Good ol' windows complicating things...
    let double_newline = if input.contains("\r") {
        "\r\n\r\n"
    } else {
        "\n\n"
    };
    let mut parts = input.split(double_newline);
    parts
        .next()
        .expect("Input not properly split")
        .lines()
        .fold(Vec::<(u64, u64)>::new(), |mut acc, line| {
            let mut range = line.split('-');
            let mut start = range
                .next()
                .expect("No min value")
                .parse::<u64>()
                .expect("Couldn't parse min");
            let mut end = range
                .next()
                .expect("No max value")
                .parse::<u64>()
                .expect("Couldn't parse max");

            // Continuously consolidate ranges, since a newly added range can "bridge" two previously
            // disconnected ranges.
            while let Some(overlapping_range_i) = acc.iter().position(|(b_start, b_end)| {
                (start <= *b_start && *b_start <= end) || (*b_start <= start && start <= *b_end)
            }) {
                let overlapping_range = acc.swap_remove(overlapping_range_i);
                start = min(overlapping_range.0, start);
                end = max(overlapping_range.1, end);
            }
            acc.push((start, end));
            acc
        })
        .iter()
        // Range is inclusive, so need to add one back in
        .fold(0, |total, range| total + range.1 - range.0 + 1)
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

    #[test]
    fn solver_03_1_works() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";

        assert_eq!(solver_03_1(input), 357);
    }

    #[test]
    fn solver_03_2_works() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";

        assert_eq!(solver_03_2(input), 3121910778619)
    }

    #[test]
    fn solver_04_1_works() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(solver_04_1(input), 13)
    }

    #[test]
    fn solver_04_2_works() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(solver_04_2(input), 43)
    }

    #[test]
    fn solver_05_1_works() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(solver_05_1(input), 3)
    }

    #[test]
    fn solver_05_2_works() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(solver_05_2(input), 14)
    }
}
