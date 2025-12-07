use std::collections::HashMap;

pub fn challenge7b(input: String) -> i64 {
    let lines = input
        .lines()
        .map(|s| s.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = lines[0]
        .iter()
        .copied()
        .enumerate()
        .find(|(_, c)| *c == b'S')
        .unwrap()
        .0;

    1 + down(&lines[1..], start as i64, 0, &mut HashMap::new())
}

fn down(lines: &[Vec<u8>], ray: i64, line: usize, cache: &mut HashMap<(usize, i64), i64>) -> i64 {
    if let Some(&v) = cache.get(&(line, ray)) {
        return v;
    }
    if ray < 0 || ray as usize >= lines[0].len() || line >= lines.len() {
        return 0;
    }
    let ret = if lines[line][ray as usize] == b'.' {
        down(lines, ray, line + 1, cache)
    } else {
        1 + down(lines, ray - 1, line + 1, cache) + down(lines, ray + 1, line + 1, cache)
    };

    cache.insert((line, ray), ret);
    ret
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge7b;

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
        assert_eq!(challenge7b(input), 40);
    }
}
