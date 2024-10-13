namespace VerflixtPuzzle.Model;

public interface IPuzzleResolutionStrategy
{
    bool IsSolved(Puzzle.Puzzle puzzle);
}

public class DefaultIsSolvedStrategy(int[] order) : IPuzzleResolutionStrategy
{
    public int[] Order { get; } = order;


    public bool IsSolved(Puzzle.Puzzle puzzle)
    {
        return IsSolved(puzzle, Order);
    }

    public bool IsSolved(Puzzle.Puzzle puzzle, int[] order)
    {
        if (puzzle == null) throw new ArgumentNullException(nameof(puzzle));

        if (!puzzle.GetTile(order[0]).Right.Fit(puzzle.GetTile(order[1]).Left))
            return false;

        if (!puzzle.GetTile(order[0]).Down.Fit(puzzle.GetTile(order[3]).Up))
            return false;

        if (!puzzle.GetTile(order[1]).Right.Fit(puzzle.GetTile(order[2]).Left))
            return false;

        if (!puzzle.GetTile(order[1]).Down.Fit(puzzle.GetTile(order[4]).Up))
            return false;

        if (!puzzle.GetTile(order[2]).Down.Fit(puzzle.GetTile(order[5]).Up))
            return false;

        if (!puzzle.GetTile(order[3]).Down.Fit(puzzle.GetTile(order[6]).Up))
            return false;

        if (!puzzle.GetTile(order[3]).Right.Fit(puzzle.GetTile(order[4]).Left))
            return false;

        if (!puzzle.GetTile(order[4]).Right.Fit(puzzle.GetTile(order[5]).Left))
            return false;

        if (!puzzle.GetTile(order[4]).Down.Fit(puzzle.GetTile(order[7]).Up))
            return false;

        if (!puzzle.GetTile(order[5]).Down.Fit(puzzle.GetTile(order[8]).Up))
            return false;

        if (!puzzle.GetTile(order[6]).Right.Fit(puzzle.GetTile(order[7]).Left))
            return false;

        if (!puzzle.GetTile(order[7]).Right.Fit(puzzle.GetTile(order[8]).Left))
            return false;

        return true;
    }
}