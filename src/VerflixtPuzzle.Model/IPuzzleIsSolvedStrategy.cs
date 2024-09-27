namespace VerflixtPuzzle.Model;

public interface IPuzzleIsSolvedStrategy
{
    bool IsSolved(Puzzle puzzle);
}

public class DefaultIsSolvedStrategy : IPuzzleIsSolvedStrategy
{
    public bool IsSolved(Puzzle puzzle)
    {
        if (puzzle == null) throw new ArgumentNullException(nameof(puzzle));

        // all tiles considered as matrix 
        // 0 1 2
        // 3 4 5
        // 6 7 8

       if(!puzzle.GetTile(0).Right.Fit(puzzle.GetTile(1).Left))
           return false;

       if (!puzzle.GetTile(0).Down.Fit(puzzle.GetTile(3).Up))
           return false;

       if (!puzzle.GetTile(1).Right.Fit(puzzle.GetTile(2).Left))
           return false;

       if (!puzzle.GetTile(1).Down.Fit(puzzle.GetTile(4).Up))
           return false;

       if (!puzzle.GetTile(2).Down.Fit(puzzle.GetTile(5).Up))
           return false;

       if (!puzzle.GetTile(3).Down.Fit(puzzle.GetTile(6).Up))
           return false;

       if (!puzzle.GetTile(3).Right.Fit(puzzle.GetTile(4).Left))
           return false;

       if (!puzzle.GetTile(4).Right.Fit(puzzle.GetTile(5).Left))
           return false;

       if (!puzzle.GetTile(4).Down.Fit(puzzle.GetTile(7).Up))
           return false;

       if (!puzzle.GetTile(5).Down.Fit(puzzle.GetTile(8).Up))
           return false;

       if (!puzzle.GetTile(6).Right.Fit(puzzle.GetTile(7).Left))
           return false;

       if (!puzzle.GetTile(7).Right.Fit(puzzle.GetTile(8).Left))
           return false;

        return true;
    }
}