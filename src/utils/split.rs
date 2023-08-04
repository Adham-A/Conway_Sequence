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
        (r_len > 1 && '1' == r_0 && Some('1') != r_1 && r_1 != r_2) // 1^1 X^1
            || (r_len > 2 && '1' == r_0 && Some('1') == r_1 && Some('1') == r_2 && Some('1') != r_3) // 1^3
            || ('3' == r_0 && (r_len < 2 || ( Some('3') != r_1 && (r_2 != r_1 || r_3 != r_1)))) // 3^1 X^!=3
            || (!['1', '2', '3'].contains(&r_0) && Some(r_0) != r_1 )
    } else if '2' != l_last && '2' == r_0 && Some('2') == r_1 {
        //...22
        (r_len > 3 && (Some('1') == r_2 && Some('1') != r_3  && r_4 != r_3)) //1^1 X^1
            || (Some('1') == r_2 && Some('1') != r_3 && r_4 != r_3) // 1^3
            || Some('3') == r_2 && (r_len < 3 || (Some('3') != r_3 && (r_4 != r_3 || r_5 != r_3))) // 3^1 X^!=3
            || (r_len == 2||( r_len > 3 && ![Some('1'), Some('2'), Some('3')].contains(&r_2) && r_2 != r_3))
    //n^0 or 1
    } else {
        false
    }
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

    #[test_case("22", "1" ; "2^2X^1")]
    #[test_case("12" , "22"; "2^3")]
    #[test_case("311222"  , "22"; "2^5")]
    #[test_case("3112"  , "22"; "2^3 bis")]

    fn string_split_22_invalid(l: &str, r: &str) {
        assert!(!is_valid_split(l, r))
    }
    #[test_case("311", "2213")]
    #[test_case("111", "22")]
    #[test_case("3", "22")]
    #[test_case("31", "22")]
    #[test_case("311", "22")]
    #[test_case("4", "22")]
    #[test_case("11", "22")]

    fn string_split_22_valid(l: &str, r: &str) {
        assert!(is_valid_split(l, r))
    }
}
