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
}
