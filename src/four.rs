
pub(crate) mod search_directions {

    pub(crate) fn horizontal(input: Vec<String>) -> usize {

        let mut cnt = 0;
        for string in input {
            cnt += string.matches("XMAS").count();
            cnt += string.matches("SAMX").count();
        }

        cnt
    }

    pub(crate) fn vertical(input: Vec<String>) -> usize {

        let vector_size = input[0].len();
        let string_size = input.len();

        let mut vecs_of_chars: Vec<Vec<char>> = vec![vec![]; vector_size];
        for string in input {
            for(j, c) in string.chars().enumerate() {
                vecs_of_chars[j].push(c);
            }
        }

        let mut transposed:Vec<String> = vec![String::with_capacity(string_size); vector_size];
        for (i, string) in transposed.iter_mut().enumerate() {
            *string = vecs_of_chars[i].clone().into_iter().collect();
        }

        horizontal(transposed)
    }

    pub(crate) fn diagonal_a(input: Vec<String>) -> usize {
        0
    }

}
#[cfg(test)]
mod tests {
    use crate::four::search_directions;
    fn get_puzzle() -> Vec<String> {
        let mut puzzle:Vec<String> = vec![];
        puzzle.push("....XXMAS.".parse().unwrap());
        puzzle.push(".SAMXMS...".parse().unwrap());
        puzzle.push("...S..A...".parse().unwrap());
        puzzle.push("..A.A.MS.X".parse().unwrap());
        puzzle.push("XMASAMX.MM".parse().unwrap());
        puzzle.push("X.....XA.A".parse().unwrap());
        puzzle.push("S.S.S.S.SS".parse().unwrap());
        puzzle.push(".A.A.A.A.A".parse().unwrap());
        puzzle.push("..M.M.M.MM".parse().unwrap());
        puzzle.push(".X.X.XMASX".parse().unwrap());

        puzzle
    }
    #[test]
    fn search_when_horizontal_then_found()
    {
        // outline
        let mut puzzle = get_puzzle();
        let expected = 5;

        // exercise
        let actual = search_directions::horizontal (puzzle);

        assert_eq!(actual, expected);
    }

    #[test]
    fn search_when_vertical_then_found()
    {
        // outline
        let mut puzzle = get_puzzle();
        let expected = 3;

        // exercise
        let actual = search_directions::vertical (puzzle);

        assert_eq!(actual, expected);
    }

    #[test]
    fn search_when_diagonal_a_then_found()
    {
        // outline
        let mut puzzle = get_puzzle();
        let expected = 3;

        // exercise
        let actual = search_directions::diagonal_a (puzzle);

        assert_eq!(actual, expected);
    }
}