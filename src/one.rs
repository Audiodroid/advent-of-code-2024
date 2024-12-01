pub(crate) fn add_list_distances(mut list_a: Vec<i32>, mut list_b: Vec<i32>) -> i32
{
    if list_a.is_empty() && list_b.is_empty() {
        return 0;
    }

    list_a.sort_unstable();
    list_b.sort_unstable();

    let mut sum = 0;
    loop {

        if list_a.is_empty() && list_b.is_empty()
        {
            return sum;
        }

        let a = list_a.pop().unwrap_or(0);
        let b = list_b.pop().unwrap_or(0);

        sum += (a - b).abs();
    }
}

#[cfg(test)]
mod tests {
    use crate::one::add_list_distances;

    #[test]
    fn when_lists_empty_then_result_is_zero()
    {
        // outline
        let list_a = vec![];
        let list_b = vec![];
        let expected = 0;

        // exercise
        let actual = add_list_distances(list_a, list_b);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn when_lists_have_one_entry_then_result_is_distance()
    {
        // outline
        let list_a = vec![1];
        let list_b = vec![2];
        let expected = 1;

        // exercise
        let actual = add_list_distances(list_a, list_b);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn when_lists_have_two_entries_then_result_is_added_distance()
    {
        // outline
        let list_a = vec![1, 1];
        let list_b = vec![2, 2];
        let expected = 2;

        // exercise
        let actual = add_list_distances(list_a, list_b);

        // evaluate
        assert_eq!(actual , expected);
    }
    #[test]
    fn when_lists_have_unordered_entries_then_result_is_added_correctly()
    {
        // outline
        let list_a = vec![4, 1];
        let list_b = vec![1, 2];
        let expected = 2;

        // exercise
        let actual = add_list_distances(list_a, list_b);

        // evaluate
        assert_eq!(actual , expected);
    }
}