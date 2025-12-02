pub fn challenge1a(input: String) -> i64 {
    let rots = input.trim().lines().map(|ln| {
        let (dir, num) = ln.split_at(1);
        num.parse::<i64>().unwrap() * if dir == "L" { -1 } else { 1 }
    });

    let mut zeros = 0;
    let mut curr = 50;
    for n in rots {
        curr += n;
        while curr < 0 {
            curr += 100;
        }
        while curr >= 100 {
            curr -= 100;
        }
        if curr == 0 {
            zeros += 1;
        }
    }

    zeros
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge1a;

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

        print!("input: {input}");
        let result = challenge1a(input.to_string());
        assert_eq!(result, 3);
    }
}
