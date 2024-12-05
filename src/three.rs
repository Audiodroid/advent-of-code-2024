use regex::Regex;

pub(crate) fn scan(input: &mut String) -> i32
{
    let re = Regex::new(r"(mul\()[\d]+(,)[\d]+(\))").unwrap();
    let mut result = 0;
    while let Some(matched) = re.find(input) {
        let re_number = Regex::new(r"[\d]+").unwrap();
        if let Some(x_match) = re_number.find(matched.as_str()) {
            let x: i32 = x_match.as_str().parse().unwrap();
            if let Some(y_match) = re_number.find(&matched.as_str().to_string()[x_match.end()..].to_string()) {
                let y: i32 = y_match.as_str().parse().unwrap();
                result += x * y;
            }
        }
        *input = input[matched.end()..].to_string();
    }

    result
}
pub(crate) fn scan_do_not_dont(input: &mut String) -> i32
{
    let mut result = 0;
    let mut begin = 0;

    while let Some(pos) = input[begin..].find("don't()") {
        let absolute_pos_begin = begin + pos;
        let mut substring = input[begin..absolute_pos_begin].to_string();
        result += scan(&mut substring);

        let absolute_pos_end = absolute_pos_begin + ("don't()").len();
        if let Some(new_pos) = input[absolute_pos_end..].find("do()") {
            begin = absolute_pos_end + new_pos + ("do()").len();
        }
        else {
            return result;
        }
    }

    result += scan(&mut input[begin..].to_string());

    result
}

#[cfg(test)]
mod tests {
    use crate::three::{scan, scan_do_not_dont};
    #[test]
    fn scan_when_nothing_found_then_returns_zero()
    {
        // outline
        let mut input = String::from("pop");
        let expected = 0;

        // exercise
        let actual = scan(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_when_one_found_then_returns_the_multiplication()
    {
        // outline
        let mut input = String::from("mul(1,1)");
        let expected = 1;

        // exercise
        let actual = scan(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_when_two_found_then_returns_the_added_multiplication()
    {
        // outline
        let mut input = String::from("mul(1,1)mul(1,1)");
        let expected = 2;

        // exercise
        let actual = scan(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_when_two_more_complex_found_then_returns_the_added_multiplication()
    {
        // outline
        let mut input = String::from("mul(2,2)mul(3,3)");
        let expected =  13;

        // exercise
        let actual = scan(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_when_two_found_with_noise_in_between_then_returns_the_added_multiplication()
    {
        // outline
        let mut input = String::from("@mul(2,2)asdwmul(2,3)");
        let expected =  10;

        // exercise
        let actual = scan(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_when_two_found_with_double_digits_and_noise_in_between_then_returns_the_added_multiplication()
    {
        // outline
        let mut input = String::from("@mul(22,2)asdwmul(22,3)");
        let expected =  110;

        // exercise
        let actual = scan(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_when_example_then_returns_correct_result()
    {
        // outline
        let mut input = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let expected =  161;

        // exercise
        let actual = scan(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_do_not_dont_when_no_dos_or_donts_then_returns_mul()
    {
        // outline
        let mut input = String::from("xmul(2,4)");
        let expected =  8;

        // exercise
        let actual = scan_do_not_dont(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_do_not_dont_when_dont_then_returns_zero()
    {
        // outline
        let mut input = String::from("don't()xmul(2,4)");
        let expected =  0;

        // exercise
        let actual = scan_do_not_dont(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_do_not_dont_when_mul_before_dont_then_returns_mul()
    {
        // outline
        let mut input = String::from("xmul(3,3)don't()xmul(2,4)");
        let expected =  9;

        // exercise
        let actual = scan_do_not_dont(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_do_not_dont_when_do_after_dont_then_returns_added_mul()
    {
        // outline
        let mut input = String::from("xmul(3,3)don't()xmul(2,4)do()mul(3,3)");
        let expected =  18;

        // exercise
        let actual = scan_do_not_dont(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_do_not_dont_when_dont_at_end_then_returns_added_mul_excluding_end()
    {
        // outline
        let mut input = String::from("xmul(3,3)don't()xmul(2,4)do()mul(3,3)don't()mul(3,3)");
        let expected =  18;

        // exercise
        let actual = scan_do_not_dont(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_do_not_dont_when_dont_without_apostrofy_then_ignored()
    {
        // outline
        let mut input = String::from("xmul(3,3)don't()xmul(2,4)do()mul(3,3)dont()mul(3,3)");
        let expected =  27;

        // exercise
        let actual = scan_do_not_dont(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn scan_do_not_dont_when_example_then_returns_correct_result()
    {
        // outline
        let mut input = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        let expected =  48;

        // exercise
        let actual = scan_do_not_dont(&mut input);

        // evaluate
        assert_eq!(actual, expected);
    }
}