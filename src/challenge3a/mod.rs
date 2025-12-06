use std::cmp::max;

pub fn challenge3a(input: String) -> i64 {
    input
        .lines()
        .map(|ln| {
            ln.chars()
                .map(|b| b as i64 - '0' as i64)
                .collect::<Vec<_>>()
        })
        .map(|ln| {
            let mut largest = 0;
            let mut smolest = 0;
            let mut currtot = 0;
            for n in ln {
                if n > smolest {
                    smolest = n;
                    currtot = max(currtot, largest * 10 + smolest);
                }
                if n > largest {
                    largest = n;
                    smolest = 0;
                }
            }
            currtot
        })
        .sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge3a;

    #[test]
    fn input() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        "};
        assert_eq!(challenge3a(input.to_string()), 357);
    }
}
