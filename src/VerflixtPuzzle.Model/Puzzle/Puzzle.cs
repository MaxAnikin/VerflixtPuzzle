using System.Text;

namespace VerflixtPuzzle.Model.Puzzle
{
    public class Puzzle
    {
        private readonly SquareTile[] _initialSquareTiles;
        private SquareTile[] _currentSquareTiles;

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
        }

        public int TilesCount => _initialSquareTiles.Length;

        public string GetPositionUniqueId(int[] SquareTilesOrder)
        {
            var builder = new StringBuilder();
            for (int i = 0; i < 9; i++)
            {
                builder.Append($"{_initialSquareTiles[SquareTilesOrder[i]].PositionId}");
            }
            return builder.ToString();
        }

        public bool IsSolved(IPuzzleIsSolvedStrategy strategy)
        {
            if (strategy == null)
                throw new ArgumentNullException(nameof(strategy));

            return strategy.IsSolved(this);
        }

        public bool IsSolved(int[] order)
        {
            return new DefaultIsSolvedStrategy(order).IsSolved(this);
        }

        public bool IsSolved(int[] order, int[] position)
        {
            return new DefaultIsSolvedStrategy(order).IsSolved(this);
        }

        public SquareTile GetTile(int index) => index switch
        {
            >= 0 and <= 8 => _currentSquareTiles[index],
            _ => throw new ArgumentOutOfRangeException(nameof(index), "Index must be between 0 and 8."),
        };

        public void Permutate(int[] order)
        {
            var temp = new SquareTile[_initialSquareTiles.Length];

            for (int j = 0; j < order.Length; j++)
            {
                temp[j] = _initialSquareTiles[order[j]];
            }

            _currentSquareTiles = temp;
        }

        public int[] GetDefaultOrder()
        {
            // all SquareTiles considered as matrix 
            // 0 1 2
            // 3 4 5
            // 6 7 8

            return Enumerable.Range(0, TilesCount).ToArray();
        }
    }
}
