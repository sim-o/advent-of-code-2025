pub fn challenge3b(input: String) -> i64 {
    input.lines().map(parse_line).map(|ln| numba(&ln)).sum()
}

fn parse_line(ln: &str) -> Vec<i64> {
    ln.chars()
        .map(|b| b as i64 - '0' as i64)
        .collect::<Vec<_>>()
}

fn numba(numbers: &[i64]) -> i64 {
    const COUNT: usize = 12;
    let mut res = [0i64; COUNT];
    res.copy_from_slice(&numbers[0..COUNT]);
    'outer: for &n in &numbers[COUNT..] {
        for i in 1..COUNT {
            if res[i] > res[i - 1] {
                res.copy_within(i..COUNT, i - 1);
                res[COUNT - 1] = n;
                continue 'outer;
            }
        }
        if n > res[COUNT - 1] {
            res[COUNT - 1] = n;
        }
    }

    res.into_iter().fold(0, |acc, n| acc * 10 + n)
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge3b::{challenge3b, numba, parse_line};

    #[test]
    fn input() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        assert_eq!(challenge3b(input.to_string()), 3121910778619);
    }

    #[test]
    fn test_numba() {
        assert_eq!(numba(&parse_line("987654321111111")), 987654321111);
        assert_eq!(numba(&parse_line("811111111111119")), 811111111119);
        assert_eq!(numba(&parse_line("234234234234278")), 434234234278);
        assert_eq!(numba(&parse_line("818181911112111")), 888911112111);
    }
}
