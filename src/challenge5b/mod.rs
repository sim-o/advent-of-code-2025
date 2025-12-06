use std::cmp::max;

pub fn challenge5b(input: String) -> i64 {
    let mut parts = input.split("\n\n");
    let ranges = parts.next().unwrap();

    let ranges = {
        let mut r = ranges
            .lines()
            .map(|v| {
                let mut r = v.split("-").map(|n| n.parse::<i64>().unwrap());
                let start = r.next().unwrap();
                let end = r.next().unwrap();
                (start, end)
            })
            .collect::<Vec<_>>();
        r.sort();

        let mut prev = 0;
        for i in 1..r.len() {
            let (ps, pe) = r[prev];
            let (cs, ce) = r[i];

            if cs - 1 <= pe {
                r[prev] = (ps, max(ce, pe));
            } else {
                prev += 1;
                r[prev] = (cs, ce);
            }
        }
        r.truncate(prev + 1);

        r
    };

    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge5b::challenge5b;

    #[test]
    fn test() {
        let input = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "}
        .to_string();

        assert_eq!(challenge5b(input), 14);
    }
}
