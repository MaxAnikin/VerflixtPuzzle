
namespace VerflixtPuzzle.Model.Puzzle
{
    //public class PuzzleWithState
    //{
    //    private readonly SquareTile[] _initialSquareTiles;
    //    private readonly Relation[] _relations;

    //    public PuzzleWithState(SquareTile[] tiles)
    //    {
    //        if (tiles == null)
    //            throw new ArgumentNullException(nameof(tiles));

    //        _initialSquareTiles = tiles;

    //        _relations = BuildRelations(tiles);
    //    }

    //    private Relation[] BuildRelations(SquareTile[] tiles)
    //    {
    //        var relations = new List<Relation>();
    //        for (int i = 0; i < tiles.Length; i++)
    //        {
    //            relations.AddRange(GetTileRelations(i));
    //        }

    //        return relations.ToArray();
    //    }

    //    private IEnumerable<Relation> GetTileRelations(int i) => i switch
    //    {
    //        0 => [new Relation(GetTile(0).Right, GetTile(1).Left), new Relation(GetTile(0).Down, GetTile(3).Up)],

    //        //1 => GetTile(1).Left.Fit(GetTile(0).Right) && GetTile(1).Down.Fit(GetTile(4).Up) && GetTile(1).Right.Fit(GetTile(2).Left),
    //        //2 => GetTile(2).Left.Fit(GetTile(1).Right) && GetTile(2).Down.Fit(GetTile(5).Up),
    //        //3 => GetTile(3).Up.Fit(GetTile(0).Down) && GetTile(3).Right.Fit(GetTile(4).Left) && GetTile(3).Down.Fit(GetTile(6).Up),
    //        //4 => GetTile(4).Left.Fit(GetTile(3).Right) && GetTile(4).Up.Fit(GetTile(1).Down) && GetTile(4).Right.Fit(GetTile(5).Left) && GetTile(4).Down.Fit(GetTile(7).Up),
    //        //5 => GetTile(5).Left.Fit(GetTile(4).Right) && GetTile(5).Up.Fit(GetTile(3).Down) && GetTile(5).Down.Fit(GetTile(8).Up),
    //        //6 => GetTile(6).Up.Fit(GetTile(3).Down) && GetTile(6).Right.Fit(GetTile(7).Left),
    //        //7 => GetTile(7).Left.Fit(GetTile(6).Right) && GetTile(7).Up.Fit(GetTile(4).Down) && GetTile(7).Right.Fit(GetTile(8).Left),
    //        //8 => GetTile(8).Left.Fit(GetTile(7).Right) && GetTile(8).Up.Fit(GetTile(5).Down),
    //        _ => []
    //    };

    //    public int TilesCount => _initialSquareTiles.Length;

    //    public SquareTile GetTile(int index) => index switch
    //    {
    //        >= 0 and <= 8 => _initialSquareTiles[index],
    //        _ => throw new ArgumentOutOfRangeException(nameof(index), "Index must be between 0 and 8."),
    //    };

    //    public void Permutate(Action<PuzzleWithState, int[]> onNewPermutationAction)
    //    {
    //        Permutate(Enumerable.Range(0, TilesCount).ToArray(), TilesCount, onNewPermutationAction);
    //    }

    //    private void Permutate(int[] order, int size, Action<PuzzleWithState, int[]> onSolvedAction)
    //    {
    //        if (size == 1)
    //        {
    //            RotateAndSolve(order, [], size, onSolvedAction);
    //            return;
    //        }

    //        for (int i = 0; i < size; i++)
    //        {
    //            Permutate(order, size - 1, onSolvedAction);

    //            if (size % 2 == 0)
    //            {
    //                (order[i], order[size - 1]) = (order[size - 1], order[i]);
    //            }
    //            else
    //            {
    //                (order[0], order[size - 1]) = (order[size - 1], order[0]);
    //            }
    //        }
    //    }

    //    public void RotateAndSolve(int[] order, int[] startCheck, int i, Action<PuzzleWithState, int[]> onSolvedAction)
    //    {
    //        for (int j = 0; j < 4; j++)
    //        {
    //            if (TilesCount > i + 1)
    //            {
    //                RotateAndSolve(order, startCheck, i + 1, onSolvedAction);
    //            }

    //            //if (CheckTile(order[i]))
    //            {
    //                if (IsSolved(order, order[i]))
    //                    onSolvedAction(this, order);
    //            }

    //            GetTile(order[i]).Rotate();
    //        }
    //    }

    //    private bool IsSolved(int[] order, int startIndex)
    //    {
    //        if (!CheckTile(startIndex))
    //            return false;

    //        for (int i = 0; i < order.Length; i++)
    //        {
    //            if (i == startIndex)
    //                continue;

    //            if (!CheckTile(i))
    //                return false;
    //        }

    //        return true;
    //    }

    //    private bool IsSolved(int[] order)
    //    {
    //        if (!GetTile(order[0]).Right.Fit(GetTile(order[1]).Left))
    //            return false;

    //        if (!GetTile(order[0]).Down.Fit(GetTile(order[3]).Up))
    //            return false;

    //        if (!GetTile(order[1]).Right.Fit(GetTile(order[2]).Left))
    //            return false;

    //        if (!GetTile(order[1]).Down.Fit(GetTile(order[4]).Up))
    //            return false;

    //        if (!GetTile(order[2]).Down.Fit(GetTile(order[5]).Up))
    //            return false;

    //        if (!GetTile(order[3]).Down.Fit(GetTile(order[6]).Up))
    //            return false;

    //        if (!GetTile(order[3]).Right.Fit(GetTile(order[4]).Left))
    //            return false;

    //        if (!GetTile(order[4]).Right.Fit(GetTile(order[5]).Left))
    //            return false;

    //        if (!GetTile(order[4]).Down.Fit(GetTile(order[7]).Up))
    //            return false;

    //        if (!GetTile(order[5]).Down.Fit(GetTile(order[8]).Up))
    //            return false;

    //        if (!GetTile(order[6]).Right.Fit(GetTile(order[7]).Left))
    //            return false;

    //        if (!GetTile(order[7]).Right.Fit(GetTile(order[8]).Left))
    //            return false;

    //        return true;
    //    }

    //    private bool CheckTile(int i) => i switch
    //    {
    //        0 => GetTile(0).Right.Fit(GetTile(1).Left) && GetTile(0).Down.Fit(GetTile(3).Up),
    //        1 => GetTile(1).Left.Fit(GetTile(0).Right) && GetTile(1).Down.Fit(GetTile(4).Up) && GetTile(1).Right.Fit(GetTile(2).Left),
    //        2 => GetTile(2).Left.Fit(GetTile(1).Right) && GetTile(2).Down.Fit(GetTile(5).Up),
    //        3 => GetTile(3).Up.Fit(GetTile(0).Down) && GetTile(3).Right.Fit(GetTile(4).Left) && GetTile(3).Down.Fit(GetTile(6).Up),
    //        4 => GetTile(4).Left.Fit(GetTile(3).Right) && GetTile(4).Up.Fit(GetTile(1).Down) && GetTile(4).Right.Fit(GetTile(5).Left) && GetTile(4).Down.Fit(GetTile(7).Up),
    //        5 => GetTile(5).Left.Fit(GetTile(4).Right) && GetTile(5).Up.Fit(GetTile(3).Down) && GetTile(5).Down.Fit(GetTile(8).Up),
    //        6 => GetTile(6).Up.Fit(GetTile(3).Down) && GetTile(6).Right.Fit(GetTile(7).Left),
    //        7 => GetTile(7).Left.Fit(GetTile(6).Right) && GetTile(7).Up.Fit(GetTile(4).Down) && GetTile(7).Right.Fit(GetTile(8).Left),
    //        8 => GetTile(8).Left.Fit(GetTile(7).Right) && GetTile(8).Up.Fit(GetTile(5).Down),
    //        _ => false
    //    };

    //    private bool CheckIsSolved()
    //    {

    //    }
    //}

    //internal class Relation
    //{
    //    private ISide right;
    //    private ISide left;

    //    public Relation(ISide right, ISide left)
    //    {
    //        this.right = right;
    //        this.left = left;
    //    }
    //}
}
