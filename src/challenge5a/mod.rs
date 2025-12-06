use std::cmp::max;

pub fn challenge5a(input: String) -> i64 {
    let mut parts = input.split("\n\n");
    let ranges = parts.next().unwrap();
    let ingred = parts.next().unwrap();

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

    let ingred = ingred
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    ingred
        .into_iter()
        .filter(|&i| ranges_contain(&ranges, i))
        .count() as i64
}

fn ranges_contain(ranges: &[(i64, i64)], ingred: i64) -> bool {
    let mut left: i64 = 0;
    let mut right: i64 = ranges.len() as i64 - 1;

    while left <= right {
        let i = (right - left) / 2 + left;
        let (start, end) = ranges[i as usize];
        let cmp = cmp_range(ingred, start, end);
        if cmp < 0 {
            right = i - 1;
        } else if cmp > 0 {
            left = i + 1;
        } else {
            return true;
        }
    }
    false
}

fn cmp_range(ingred: i64, start: i64, end: i64) -> i32 {
    if ingred < start {
        -1
    } else if ingred > end {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge5a::challenge5a;

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

        assert_eq!(challenge5a(input), 3);
    }
}
