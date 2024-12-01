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

pub(crate) fn add_similarity_score(mut list_a: Vec<i32>, list_b: Vec<i32>) -> i32
{
    let mut sum = 0;
    loop
    {
        if list_a.is_empty()
        {
            return sum;
        }

        let a = list_a.pop().unwrap_or(0);
        let v: Vec<&i32> = list_b.iter().filter(|&x| *x == a).collect();
        sum += (v.len() as i32) * a;
    }
}

#[cfg(test)]
mod tests {
    use crate::one::{add_list_distances, add_similarity_score};

    // add_list_distance
    #[test]
    fn add_list_distances_when_lists_empty_then_result_is_zero()
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
    fn add_list_distances_when_lists_have_one_entry_then_result_is_distance()
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
    fn add_list_distances_when_lists_have_two_entries_then_result_is_added_distance()
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
    fn add_list_distances_when_lists_have_unordered_entries_then_result_is_added_correctly()
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

    // add_similarity_score
    #[test]
    fn add_similarity_score_when_lists_empty_then_result_is_zero()
    {
        // outline
        let list_a = vec![];
        let list_b = vec![];
        let expected = 0;

        // exercise
        let actual = add_similarity_score(list_a, list_b);

        // evaluate
        assert_eq!(actual, expected);
    }
    #[test]
    fn add_similarity_score_when_lists_both_with_one_then_result_is_one()
    {
        // outline
        let list_a = vec![1];
        let list_b = vec![1];
        let expected = 1;

        // exercise
        let actual = add_similarity_score(list_a, list_b);

        // evaluate
        assert_eq!(actual , expected);
    }

    #[test]
    fn add_similarity_score_when_lists_both_with_one_and_two_then_result_is_three()
    {
        // outline
        let list_a = vec![1, 2];
        let list_b = vec![1, 2];
        let expected = 3;

        // exercise
        let actual = add_similarity_score(list_a, list_b);

        // evaluate
        assert_eq!(actual , expected);
    }
}