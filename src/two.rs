
fn is_valid(row: &Vec<i32>) -> bool
{
    let ascending = row[1] > row[0];
    let mut prev = row[0];
    for x in &row[1..]
    {
        if ascending && (*x < prev) { return false; }
        else if !ascending && (*x > prev) { return false; }
        else if (*x - prev).abs() > 3 { return false; }
        else if (*x - prev).abs() == 0 { return false; }

        prev = *x;
    }

    true
}

pub(crate) fn count_valid_reports(rows: Vec<Vec<i32>>, with_tolerance: bool) -> usize
{
    let mut cnt = 0;
    for row in rows {
        if is_valid(&row)
        {
            cnt += 1;
        }
        else if with_tolerance
        {
            let positions = row.len();
            for pos in 0..=positions-1
            {
                let short_row = [&row[..pos], &row[pos+1..]].concat();
                if is_valid(&short_row)
                {
                    cnt +=1;
                    break;
                }
            }
        }
    }

    cnt
}

#[cfg(test)]
mod tests {
    use crate::two::{count_valid_reports};
    #[test]
    fn count_valid_reports_when_list_empty_then_result_is_zero()
    {
        // outline
        let rows = vec![];
        let expected = 0;

        // exercise
        let actual = count_valid_reports(rows, false);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn count_valid_reports_when_row_is_asc_then_return_one()
    {
        // outline
        let row = vec![1, 2, 3, 4, 5, 6, 7];
        let rows = vec![row];
        let expected = 1;

        // exercise
        let actual = count_valid_reports(rows, false);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn count_valid_reports_when_row_is_not_asc_then_return_zero()
    {
        // outline
        let row = vec![1, 2, 1, 4, 5, 6, 7];
        let rows = vec![row];
        let expected = 0;

        // exercise
        let actual = count_valid_reports(rows, false);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn count_valid_reports_when_row_is_desc_then_return_one()
    {
        // outline
        let row = vec![7, 6, 5, 4, 3, 2, 1];
        let rows = vec![row];
        let expected = 1;

        // exercise
        let actual = count_valid_reports(rows, false);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn count_valid_reports_when_row_is_not_desc_then_return_one()
    {
        // outline
        let row = vec![7, 6, 7, 4, 3, 2, 1];
        let rows = vec![row];
        let expected = 0;

        // exercise
        let actual = count_valid_reports(rows, false);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn count_valid_reports_when_too_much_differ_then_return_zero()
    {
        // outline
        let row = vec![1, 2, 3, 4, 5, 6, 10];
        let rows = vec![row];
        let expected = 0;

        // exercise
        let actual = count_valid_reports(rows, false);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn count_valid_reports_when_numbers_do_not_differ_then_return_zero()
    {
        // outline
        let row = vec![8, 6, 4, 4, 1];
        let rows = vec![row];
        let expected = 0;

        // exercise
        let actual = count_valid_reports(rows, false);

        // evaluate
        assert_eq!(actual , expected);
    }

    #[test]
    fn with_tolerance_when_1st_line_answer_is_still_true()
    {
        let row = vec![7, 6, 4, 2, 1];

        let rows = vec![row];
        let expected = 1;

        // exercise
        let actual = count_valid_reports(rows, true);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn with_tolerance_when_2nd_line_answer_is_still_true()
    {
        let row = vec![1, 2, 7, 8, 9];

        let rows = vec![row];
        let expected = 0;

        // exercise
        let actual = count_valid_reports(rows, true);

        // evaluate
        assert_eq!(actual , expected);
    }
}
