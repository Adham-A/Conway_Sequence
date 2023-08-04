pub fn is_valid_split(l: &str, r: &str) -> bool {
    let l_len = l.len();
    let r_len = r.len();

    if l_len == 0 || r_len == 0 {
        return true;
    }

    let l_last = l.chars().last().unwrap();
    let mut r_chars = r.chars();

    let r_0 = r_chars.next().unwrap();

    if !['1', '2', '3'].contains(&l_last) && ['1', '2', '3'].contains(&r_0) {
        return true;
    }

    let r_1 = r_chars.next();
    let r_2 = r_chars.next();
    let r_3 = r_chars.next();
    let r_4 = r_chars.next();
    let r_5 = r_chars.next();

    if '2' == l_last {
        //2...
        is_1_1_x_1(r_len, 1, Some(r_0), r_1, r_2)
            || is_1_3(Some(r_0), r_1, r_2, r_3)
            || is_3_1_x_not_3(r_len, 2, Some(r_0), r_1, r_2, r_3)
            || (!['1', '2', '3'].contains(&r_0) && Some(r_0) != r_1)
    } else if '2' != l_last && '2' == r_0 && Some('2') == r_1 {
        //...22
        is_1_1_x_1(r_len, 3, r_2, r_3, r_4)
            || is_1_3(r_2, r_3, r_4, r_5)
            || is_3_1_x_not_3(r_len, 3, r_2, r_3, r_4, r_5)
            || (r_len == 2
                || (r_len > 3 && ![Some('1'), Some('2'), Some('3')].contains(&r_2) && r_2 != r_3))
    //n^0 or 1
    } else {
        false
    }
}

pub fn is_1_1_x_1(
    string_len: usize,
    goal_len: usize,
    char_0: Option<char>,
    char_1: Option<char>,
    char_2: Option<char>,
) -> bool {
    let o_1 = Some('1');
    string_len > goal_len && (o_1 == char_0 && o_1 != char_1 && char_2 != char_1)
}

pub fn is_1_3(
    char_0: Option<char>,
    char_1: Option<char>,
    char_2: Option<char>,
    char_3: Option<char>,
) -> bool {
    let o_1 = Some('1');
    o_1 == char_0 && o_1 == char_1 && o_1 == char_2 && char_3 != o_1
}

pub fn is_3_1_x_not_3(
    string_len: usize,
    goal_len: usize,
    char_0: Option<char>,
    char_1: Option<char>,
    char_2: Option<char>,
    char_3: Option<char>,
) -> bool {
    //             || ('3' == r_0 && (r_len < 2 || ( Some('3') != r_1 && (r_2 != r_1 || r_3 != r_1)))) // 3^1 X^!=3
    let o_3 = Some('3');
    o_3 == char_0
        && (string_len < goal_len || (o_3 != char_1 && (char_2 != char_1 || char_3 != char_1)))
}
#[cfg(test)]
mod tests {
    use crate::split::is_valid_split;
    use test_case::test_case;
    #[test_case("", "1" ; "Left string empty")]
    #[test_case("1",  ""  ; "Right string string empty")]
    #[test_case("",  ""  ; "Left and Right strings empty")]
    fn string_split_empty(l: &str, r: &str) {
        assert!(is_valid_split(l, r))
    }

    #[test_case("2", "12")]
    #[test_case("2", "111")]
    #[test_case("2", "1112")]
    #[test_case("2", "3")]
    #[test_case("2", "31")]
    #[test_case("2", "311")]
    #[test_case("2", "3112")]
    #[test_case("2", "311222")]
    #[test_case("2", "4")]
    #[test_case("22", "12")]

    fn string_split_2_valid(l: &str, r: &str) {
        assert!(is_valid_split(l, r))
    }

    #[test_case("2", "1111")]
    #[test_case("2", "3111")]
    #[test_case("2", "33")]
    #[test_case("2", "333")]
    #[test_case("2", "3333")]
    #[test_case("2", "31111")]
    #[test_case("2", "44")]
    #[test_case("22", "11")]

    fn string_split_2_invalid(l: &str, r: &str) {
        assert!(!is_valid_split(l, r))
    }

    #[test_case("311", "2213")]
    #[test_case("111", "22")]
    #[test_case("2111", "22")]
    #[test_case("3", "22")]
    #[test_case("31", "22")]
    #[test_case("311", "22")]
    #[test_case("4", "22")]
    #[test_case("11", "22")]
    #[test_case("1111", "22")]
    #[test_case("1", "22111")]
    #[test_case("111", "22111")]
    #[test_case("1111", "22111")]

    fn string_split_22_valid(l: &str, r: &str) {
        assert!(is_valid_split(l, r))
    }

    #[test_case("22", "1")]
    #[test_case("12", "22")]
    #[test_case("311222", "22")]
    #[test_case("3112", "22")]
    #[test_case("1", "221111")]
    #[test_case("1111", "221111")]

    fn string_split_22_invalid(l: &str, r: &str) {
        assert!(!is_valid_split(l, r))
    }
}
