pub fn challenge2b(input: String) -> i64 {
    input
        .trim()
        .lines()
        .flat_map(|ln| ln.split_terminator(","))
        .flat_map(|ln| {
            let mut parts = ln.split("-").map(|i| i.parse::<i64>().unwrap());
            let first = parts.next().unwrap();
            let last = parts.next().unwrap();
            first..=last
        })
        .filter(|n| !is_val(n.to_string().as_str()))
        .sum()
}

fn is_val(input: &str) -> bool {
    let b = input.as_bytes();
    let len = b.len();
    'outer: for i in 0..=len / 2 {
        if len.is_multiple_of(i) {
            for j in (i..len).step_by(i) {
                for k in 0..i {
                    if b[k] != b[j + k] {
                        continue 'outer;
                    }
                }
            }
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    use crate::challenge2b::{challenge2b, is_val};

    #[test]
    fn is_valid() {
        assert!(!is_val("11"));
        assert!(!is_val("1212"));
        assert!(!is_val("121212"));

        assert!(is_val("1"));
        assert!(is_val("12"));
        assert!(is_val("121213"));
        assert!(is_val("12121"));
    }

    #[test]
    fn test_input() {
        let input = indoc! {"
            11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
            1698522-1698528,446443-446449,38593856-38593862,565653-565659,
            824824821-824824827,2121212118-2121212124
            "}
        .to_string();
        assert_eq!(challenge2b(input), 4174379265);
    }
}
