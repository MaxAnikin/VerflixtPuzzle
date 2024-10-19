namespace VerflixtPuzzle.Model.Puzzle
{
    public class Puzzle
    {
        public bool IsSolved { get; private set; }

        private readonly DefaultIsSolvedStrategy _resolutionStrategy;
        private readonly SquareTile[] _initialSquareTiles;
        private readonly SquareTile[] _currentSquareTiles;

        public Puzzle(SquareTile[] tiles)
        {
            if (tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles be 9.");
            
            _initialSquareTiles = new SquareTile[tiles.Length];
            tiles.CopyTo(_initialSquareTiles, 0);

            _currentSquareTiles = new SquareTile[tiles.Length];
            tiles.CopyTo(_currentSquareTiles, 0);

            _resolutionStrategy = new DefaultIsSolvedStrategy(Enumerable.Range(0, TilesCount).ToArray());

            IsSolved = CheckIsSolved();
        }

        public int TilesCount => _initialSquareTiles.Length;

        public SquareTile GetTile(int index) => index switch
        {
            >= 0 and <= 8 => _currentSquareTiles[index],
            _ => throw new ArgumentOutOfRangeException(nameof(index), "Index must be between 0 and 8."),
        };

        public void Permutate(Action<Puzzle, int[]> onNewPermutationAction)
        {
            Permutate(Enumerable.Range(0, TilesCount).ToArray(), TilesCount, onNewPermutationAction);
        }

        private void Permutate(int[] order, int size, Action<Puzzle, int[]> onNewPermutationAction)
        {
            if (size == 1)
            {
                onNewPermutationAction(this, order);
                return;
            }

            for (int i = 0; i < size; i++)
            {
                Permutate(order, size - 1, onNewPermutationAction);

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

        public void Swap(int index1, int index2)
        {
            (_currentSquareTiles[index1], _currentSquareTiles[index2]) = (_currentSquareTiles[index2], _currentSquareTiles[index1]);
        }

        public void RotateAndSolve(int[] order, int[] startCheck, int i, Action<Puzzle, int[]> onSolvedAction)
        {
            for (int j = 0; j < 4; j++)
            {
                if (TilesCount > i + 1)
                {
                    RotateAndSolve(order, startCheck, i + 1, onSolvedAction);
                }

                if (_resolutionStrategy.IsSolved(this, order))
                {
                    onSolvedAction(this, order);
                }

                GetTile(order[i]).Rotate();
            }
        }

        private bool CheckTile(int i) => i switch
        {
            0 => GetTile(0).Right.Fit(GetTile(1).Left) && GetTile(0).Down.Fit(GetTile(3).Up),
            1 => GetTile(1).Left.Fit(GetTile(0).Right) && GetTile(1).Down.Fit(GetTile(4).Up) && GetTile(1).Right.Fit(GetTile(2).Left),
            2 => GetTile(2).Left.Fit(GetTile(1).Right) && GetTile(2).Down.Fit(GetTile(5).Up),
            3 => GetTile(3).Up.Fit(GetTile(0).Down) && GetTile(3).Right.Fit(GetTile(4).Left) && GetTile(3).Down.Fit(GetTile(6).Up),
            4 => GetTile(4).Left.Fit(GetTile(3).Right) && GetTile(4).Up.Fit(GetTile(1).Down) && GetTile(4).Right.Fit(GetTile(5).Left) && GetTile(4).Down.Fit(GetTile(7).Up),
            5 => GetTile(5).Left.Fit(GetTile(4).Right) && GetTile(5).Up.Fit(GetTile(3).Down) && GetTile(5).Down.Fit(GetTile(8).Up),
            6 => GetTile(6).Up.Fit(GetTile(3).Down) && GetTile(6).Right.Fit(GetTile(7).Left),
            7 => GetTile(7).Left.Fit(GetTile(6).Right) && GetTile(7).Up.Fit(GetTile(4).Down) && GetTile(7).Right.Fit(GetTile(8).Left),
            8 => GetTile(8).Left.Fit(GetTile(7).Right) && GetTile(8).Up.Fit(GetTile(5).Down),
            _ => false
        };

        private bool CheckIsSolved()
        {
            for (int i = 0; i < _currentSquareTiles.Length; i++)
            {
                if(!CheckTile(i))
                    return false;
            }

            return true;
        }
    }
}
