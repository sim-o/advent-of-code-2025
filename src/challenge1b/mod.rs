pub fn challenge1b(input: String) -> i64 {
    let rots = input.trim().lines().map(|ln| {
        let (dir, num) = ln.split_at(1);
        num.parse::<i64>().unwrap() * if dir == "L" { -1 } else { 1 }
    });

    let mut zeros = 0;
    let mut curr = 50;
    for mut n in rots {
        while n > 0 {
            curr += 1;
            n -= 1;

            if curr >= 100 {
                curr -= 100;
            }
            if curr == 0 {
                zeros += 1;
            }
        }
        while n < 0 {
            curr -= 1;
            n += 1;

            if curr < 0 {
                curr += 100;
            }
            if curr == 0 {
                zeros += 1;
            }
        }
    }

    zeros
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge1b::challenge1b;

    #[test]
    fn test() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82
        "};

        let result = challenge1b(input.to_string());
        assert_eq!(result, 6);
    }

    #[test]
    fn to0() {
        let input = indoc! {"
            L50
            L200
        "};

        let result = challenge1b(input.to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn cross100() {
        let input = indoc! {"
            L49
            L2
        "};

        let result = challenge1b(input.to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn cross100thrice() {
        let input = indoc! {"
            R49
            R200
        "};

        let result = challenge1b(input.to_string());
        assert_eq!(result, 2);
    }

    #[test]
    fn onethousand() {
        let input = indoc! {"
            L50
            L1000
        "};

        let result = challenge1b(input.to_string());
        assert_eq!(result, 11);
    }
}
