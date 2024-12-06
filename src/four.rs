
pub(crate) fn search_for_xmas(input: Vec<&str>) -> i32
{
    0
}
#[cfg(test)]
mod tests {
    use crate::four::search_for_xmas;

    fn get_puzzle() -> Vec<&str> {
        let mut puzzle:Vec<&str> = vec![];
        puzzle.push("....XXMAS.");
        puzzle.push(".SAMXMS...");
        puzzle.push("...S..A...");
        puzzle.push("..A.A.MS.X");
        puzzle.push("XMASAMX.MM");
        puzzle.push("X.....XA.A");
        puzzle.push("S.S.S.S.SS");
        puzzle.push(".A.A.A.A.A");
        puzzle.push("..M.M.M.MM");
        puzzle.push(".X.X.XMASX");

        puzzle
    }
    #[test]
    fn search_when_forward_then_found()
    {
        let mut puzzle = get_puzzle();
        let actual = search_for_xmas(puzzle);
    }
}