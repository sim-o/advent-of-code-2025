pub fn challenge6b(input: String) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let (&operations, lines) = lines.split_last().unwrap();
    let regex = regex::Regex::new(r#"\s+"#).unwrap();
    let operations = regex.split(operations.trim()).collect::<Vec<_>>();

    let matrix = lines
        .iter()
        .map(|ln| ln.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // transpose
    let len_x = matrix.len();
    let len_y = matrix[0].len();
    let mut next = Vec::with_capacity(len_y);
    for i in 0..len_y {
        let mut ln = String::new();
        for j in 0..len_x {
            ln.push(matrix[j][i] as char);
        }

        next.push(ln);
    }

    let mut sum = 0;
    for (i, group) in next.split(|v| v.trim().is_empty()).enumerate() {
        let mul = operations[i] == "*";
        let init = if mul { 1 } else { 0 };
        let n = group
            .iter()
            .map(|s| s.trim().parse::<i64>().unwrap())
            .fold(init, |acc, v| if mul { acc * v } else { acc + v });
        sum += n;
    }

    sum
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
