pub fn challenge4a(input: String) -> i64 {
    let f = input
        .lines()
        .map(|ln| ln.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut count = 0;
    for (i, ln) in f.iter().enumerate() {
        for (j, &v) in ln.iter().enumerate() {
            if v {
                count += if check(&f, i, j) { 1 } else { 0 };
            }
        }
    }
    count
}

fn check(f: &[Vec<bool>], i: usize, j: usize) -> bool {
    let neighbours = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut count = 0;
    for (ii, jj) in neighbours {
        let next_i = i as i64 + ii;
        let next_j = j as i64 + jj;
        if next_i >= 0 && next_i < f.len() as i64 && next_j >= 0 && next_j < f[i].len() as i64 {
            count += if f[next_i as usize][next_j as usize] {
                1
            } else {
                0
            };
        }
    }
    count < 4
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge4a;

    #[test]
    fn test() {
        let input = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "}
        .to_string();
        assert_eq!(challenge4a(input), 13);
    }
}
