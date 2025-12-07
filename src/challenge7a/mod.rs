use std::mem::swap;

pub fn challenge7a(input: String) -> i64 {
    let lines = input
        .lines()
        .map(|s| s.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut splits = 0;
    let mut rays = vec![0u8; lines[0].len()];
    let mut next = vec![0u8; lines[0].len()];
    let start = lines[0]
        .iter()
        .copied()
        .enumerate()
        .find(|(_, c)| *c == b'S')
        .unwrap()
        .0;
    rays[start] = 1;
    for line in &lines[1..] {
        for (i, &c) in line.iter().enumerate() {
            if rays[i] == 1 {
                if c == b'.' {
                    next[i] = 1;
                }
                if c == b'^' && rays[i] == 1 {
                    splits += 1;
                    if i > 0 {
                        next[i - 1] = 1;
                    }
                    if i + 1 < next.len() {
                        next[i + 1] = 1;
                    }
                }
            }
        }

        swap(&mut rays, &mut next);
        next.fill(0);
    }

    splits
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge7a;

    #[test]
    fn test() {
        let input = indoc! {"
            .......S.......
            ...............
            .......^.......
            ...............
            ......^.^......
            ...............
            .....^.^.^.....
            ...............
            ....^.^...^....
            ...............
            ...^.^...^.^...
            ...............
            ..^...^.....^..
            ...............
            .^.^.^.^.^...^.
            ...............
        "}
        .to_string();
        assert_eq!(challenge7a(input), 21);
    }
}
