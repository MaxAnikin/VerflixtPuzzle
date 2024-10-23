namespace VerflixtPuzzle.Model.Puzzle
{
    public class Puzzle
    {
        private readonly SquareTile[] _initialSquareTiles;
        private readonly Dictionary<string, bool> _solvedCrossIds;

        public Puzzle(SquareTile[] tiles)
        {
            if (tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles be 9.");

            _initialSquareTiles = new SquareTile[tiles.Length];
            _solvedCrossIds = new Dictionary<string, bool>();

            tiles.CopyTo(_initialSquareTiles, 0);
        }

        public int TilesCount => _initialSquareTiles.Length;

        public SquareTile GetTile(int index) => index switch
        {
            >= 0 and <= 8 => _initialSquareTiles[index],
            _ => throw new ArgumentOutOfRangeException(nameof(index), "Index must be between 0 and 8."),
        };

        public void Permutate(Action<Puzzle, int[]> onNewPermutationAction)
        {
            Permutate(Enumerable.Range(0, TilesCount).ToArray(), TilesCount, onNewPermutationAction);
        }

        private void Permutate(int[] order, int size, Action<Puzzle, int[]> onSolvedAction)
        {
            if (size == 1)
            {
                RotateAndSolveCross(order, [], size, onSolvedAction);
                return;
            }

            for (int i = 0; i < size; i++)
            {
                Permutate(order, size - 1, onSolvedAction);

                if (size % 2 == 0)
                {
                    (order[i], order[size - 1]) = (order[size - 1], order[i]);
                }
                else
                {
                    (order[0], order[size - 1]) = (order[size - 1], order[0]);
                }
            }
        }

        public void RotateAndSolve(int[] order, int[] startCheck, int i, Action<Puzzle, int[]> onSolvedAction)
        {
            for (int j = 0; j < 4; j++)
            {
                if (order.Length > i + 1)
                {
                    RotateAndSolve(order, startCheck, i + 1, onSolvedAction);
                }

                //if (CheckTile(order[i]))
                {
                    if (IsSolved(order, order[i]))
                        onSolvedAction(this, order);
                }

                GetTile(order[i]).Rotate();
            }
        }

        public void RotateAndSolveCross(int[] order, int[] startCheck, int i, Action<Puzzle, int[]> onSolvedAction)
        {
            string crossId = GetOrderId(order);

            bool existingCross = _solvedCrossIds.TryGetValue(crossId, out bool isSolved);
            if (existingCross)
                return;

            if (!RotateAndSolveCrossOnly(order, [order[1], order[3], order[4], order[5], order[7]], 0))
            {
                AddOrderSolutionResult(order, false);
                return;
            }

            if (RotateAndSolveRest(order, [order[0], order[2], order[6], order[8]], 0))
            {
                AddOrderSolutionResult(order, true);
                onSolvedAction(this, order);
            }
            else AddOrderSolutionResult(order, false);
        }

        private void AddOrderSolutionResult(int[] order, bool isSolved)
        {
            //_solvedCrossIds.Add(GetCrossId(order), isSolved);
            //_solvedCrossIds.Add(string.Join(",", new[] { order[3], order[7], order[4], order[1], order[5] }), isSolved);
            //_solvedCrossIds.Add(string.Join(",", new[] { order[7], order[5], order[4], order[3], order[1] }), isSolved);
            //_solvedCrossIds.Add(string.Join(",", new[] { order[5], order[1], order[4], order[7], order[3] }), isSolved);
            _solvedCrossIds.Add(string.Join(",", new[] { order[0], order[1], order[2], order[3], order[4], order[5], order[6], order[7], order[8] }), isSolved);
            _solvedCrossIds.Add(string.Join(",", new[] { order[6], order[3], order[0], order[7], order[4], order[1], order[8], order[5], order[2] }), isSolved);
            _solvedCrossIds.Add(string.Join(",", new[] { order[8], order[7], order[6], order[5], order[4], order[3], order[2], order[1], order[0] }), isSolved);
            _solvedCrossIds.Add(string.Join(",", new[] { order[2], order[5], order[8], order[1], order[4], order[7], order[0], order[3], order[6] }), isSolved);
        }

        private string GetCrossId(int[] order)
        {
            return string.Join(",", new[] { order[1], order[3], order[4], order[5], order[7] });
        }

        private string GetOrderId(int[] order)
        {
            return string.Join(",", new[] { order[0], order[1], order[2], order[3], order[4], order[5], order[6], order[7], order[8] });
        }

        private bool RotateAndSolveRest(int[] order, int[] restOrder, int i)
        {
            for (int j = 0; j < 4; j++)
            {
                if (restOrder.Length > i + 1)
                {
                    if (RotateAndSolveRest(order, restOrder, i + 1))
                        return true;
                }

                if (IsRestSolved(order))
                    return true;

                GetTile(restOrder[i]).Rotate();
            }

            return false;
        }

        private bool RotateAndSolveCrossOnly(int[] order, int[] crossOrder, int i)
        {
            for (int j = 0; j < 4; j++)
            {
                if (crossOrder.Length > i + 1)
                {
                    if(RotateAndSolveCrossOnly(order, crossOrder, i + 1))
                        return true;
                }

                if (IsCrossSolved(order))
                    return true;

                GetTile(crossOrder[i]).Rotate();
            }

            return false;
        }

        private bool IsRestSolved(int[] order)
        {
            if (!GetTile(order[0]).Down.Fit(GetTile(order[3]).Up))
                return false;

            if (!GetTile(order[0]).Right.Fit(GetTile(order[1]).Left))
                return false;

            if (!GetTile(order[1]).Right.Fit(GetTile(order[2]).Left))
                return false;

            if (!GetTile(order[2]).Down.Fit(GetTile(order[5]).Up))
                return false;

            if (!GetTile(order[3]).Down.Fit(GetTile(order[6]).Up))
                return false;

            if (!GetTile(order[5]).Down.Fit(GetTile(order[8]).Up))
                return false;

            if (!GetTile(order[6]).Right.Fit(GetTile(order[7]).Left))
                return false;

            if (!GetTile(order[7]).Right.Fit(GetTile(order[8]).Left))
                return false;

            return true;
        }

        private bool IsCrossSolved(int[] order)
        {
            if (!GetTile(order[1]).Down.Fit(GetTile(order[4]).Up))
                return false;

            if (!GetTile(order[3]).Right.Fit(GetTile(order[4]).Left))
                return false;

            if (!GetTile(order[4]).Right.Fit(GetTile(order[5]).Left))
                return false;

            if (!GetTile(order[4]).Down.Fit(GetTile(order[7]).Up))
                return false;

            return true;
        }

        private bool IsSolved(int[] order, int startIndex)
        {
            if (!CheckTile(startIndex))
                return false;

            for (int i = 0; i < order.Length; i++)
            {
                if (i == startIndex)
                    continue;

                if (!CheckTile(i))
                    return false;
            }

            return true;
        }

        private bool IsSolved(int[] order)
        {
            if (!GetTile(order[0]).Right.Fit(GetTile(order[1]).Left))
                return false;

            if (!GetTile(order[0]).Down.Fit(GetTile(order[3]).Up))
                return false;

            if (!GetTile(order[1]).Right.Fit(GetTile(order[2]).Left))
                return false;

            if (!GetTile(order[1]).Down.Fit(GetTile(order[4]).Up))
                return false;

            if (!GetTile(order[2]).Down.Fit(GetTile(order[5]).Up))
                return false;

            if (!GetTile(order[3]).Down.Fit(GetTile(order[6]).Up))
                return false;

            if (!GetTile(order[3]).Right.Fit(GetTile(order[4]).Left))
                return false;

            if (!GetTile(order[4]).Right.Fit(GetTile(order[5]).Left))
                return false;

            if (!GetTile(order[4]).Down.Fit(GetTile(order[7]).Up))
                return false;

            if (!GetTile(order[5]).Down.Fit(GetTile(order[8]).Up))
                return false;

            if (!GetTile(order[6]).Right.Fit(GetTile(order[7]).Left))
                return false;

            if (!GetTile(order[7]).Right.Fit(GetTile(order[8]).Left))
                return false;

            return true;
        }

        private bool CheckTile(int i) => i switch
        {
            0 => GetTile(i).Right.Fit(GetTile(1).Left) && GetTile(i).Down.Fit(GetTile(3).Up),
            1 => GetTile(i).Left.Fit(GetTile(0).Right) && GetTile(i).Down.Fit(GetTile(4).Up) && GetTile(i).Right.Fit(GetTile(2).Left),
            2 => GetTile(i).Left.Fit(GetTile(1).Right) && GetTile(i).Down.Fit(GetTile(5).Up),
            3 => GetTile(i).Up.Fit(GetTile(0).Down) && GetTile(i).Right.Fit(GetTile(4).Left) && GetTile(i).Down.Fit(GetTile(6).Up),
            4 => GetTile(i).Left.Fit(GetTile(3).Right) && GetTile(i).Up.Fit(GetTile(1).Down) && GetTile(i).Right.Fit(GetTile(5).Left) && GetTile(i).Down.Fit(GetTile(7).Up),
            5 => GetTile(i).Left.Fit(GetTile(4).Right) && GetTile(i).Up.Fit(GetTile(3).Down) && GetTile(i).Down.Fit(GetTile(8).Up),
            6 => GetTile(i).Up.Fit(GetTile(3).Down) && GetTile(i).Right.Fit(GetTile(7).Left),
            7 => GetTile(i).Left.Fit(GetTile(6).Right) && GetTile(i).Up.Fit(GetTile(4).Down) && GetTile(i).Right.Fit(GetTile(8).Left),
            8 => GetTile(i).Left.Fit(GetTile(7).Right) && GetTile(i).Up.Fit(GetTile(5).Down),
            _ => false
        };

        private bool CheckIsSolved()
        {
            for (int i = 0; i < _initialSquareTiles.Length; i++)
            {
                if (!CheckTile(i))
                    return false;
            }

            return true;
        }
    }
}
