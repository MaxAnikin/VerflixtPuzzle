namespace VerflixtPuzzle.Model;

public interface IPuzzleIsSolvedStrategy
{
    bool IsSolved(Puzzle puzzle);
    
    bool IsSolved(Puzzle puzzle, int[] tilesOrder);
}

public class DefaultIsSolvedStrategy : IPuzzleIsSolvedStrategy
{
    public bool IsSolved(Puzzle puzzle)
    {
        // all tiles considered as matrix by default
        // tilesOrder[0] tilesOrder[1] tilesOrder[2]
        // tilesOrder[3] tilesOrder[4] tilesOrder[5]
        // tilesOrder[6] tilesOrder[7] tilesOrder[8]

        return IsSolved(puzzle, puzzle.GetDefaultOrder());
    }

    public bool IsSolved(Puzzle puzzle, int[] tilesOrder)
    {
        if (puzzle == null) throw new ArgumentNullException(nameof(puzzle));


        if (!puzzle.GetTile(tilesOrder[0]).Right.Fit(puzzle.GetTile(tilesOrder[1]).Left))
            return false;

        if (!puzzle.GetTile(tilesOrder[0]).Down.Fit(puzzle.GetTile(tilesOrder[3]).Up))
            return false;

        if (!puzzle.GetTile(tilesOrder[1]).Right.Fit(puzzle.GetTile(tilesOrder[2]).Left))
            return false;

        if (!puzzle.GetTile(tilesOrder[1]).Down.Fit(puzzle.GetTile(tilesOrder[4]).Up))
            return false;

        if (!puzzle.GetTile(tilesOrder[2]).Down.Fit(puzzle.GetTile(tilesOrder[5]).Up))
            return false;

        if (!puzzle.GetTile(tilesOrder[3]).Down.Fit(puzzle.GetTile(tilesOrder[6]).Up))
            return false;

        if (!puzzle.GetTile(tilesOrder[3]).Right.Fit(puzzle.GetTile(tilesOrder[4]).Left))
            return false;

        if (!puzzle.GetTile(tilesOrder[4]).Right.Fit(puzzle.GetTile(tilesOrder[5]).Left))
            return false;

        if (!puzzle.GetTile(tilesOrder[4]).Down.Fit(puzzle.GetTile(tilesOrder[7]).Up))
            return false;

        if (!puzzle.GetTile(tilesOrder[5]).Down.Fit(puzzle.GetTile(tilesOrder[8]).Up))
            return false;

        if (!puzzle.GetTile(tilesOrder[6]).Right.Fit(puzzle.GetTile(tilesOrder[7]).Left))
            return false;

        if (!puzzle.GetTile(tilesOrder[7]).Right.Fit(puzzle.GetTile(tilesOrder[8]).Left))
            return false;

        return true;
    }
}