pub fn challenge6b(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let (&operations, lines) = lines.split_last().unwrap();
    let regex = regex::Regex::new(r#"\s+"#).unwrap();
    let operations = regex.split(operations.trim()).collect::<Vec<_>>();
    let lines = lines
        .iter()
        .map(|ln| {
            regex
                .split(ln.trim())
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut results = operations
        .iter()
        .map(|&c| if c == "*" { 1 } else { 0 })
        .collect::<Vec<_>>();
    for (j, &op) in operations.iter().enumerate() {
        if op == "*" {
            for ln in &lines {
                results[j] *= ln[j];
            }
        } else {
            for ln in &lines {
                results[j] += ln[j];
            }
        }
    }

    results.iter().sum()
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge6b::challenge6b;

    #[test]
    fn test() {
        let input = indoc! {"
            123 328  51 64 
            45 64  387 23 
            6 98  215 314
            *   +   *   +  
        "}
        .to_string();
        assert_eq!(challenge6b(input), 3263827);
    }
}
