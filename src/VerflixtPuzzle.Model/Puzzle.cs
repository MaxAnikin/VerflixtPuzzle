using System.Text;

namespace VerflixtPuzzle.Model
{
    public sealed class Puzzle
    {
        private readonly Tile[] _initialTiles;
        private Tile[] _currentTiles;

        public Puzzle(Tile[] tiles)
        {
            if(tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles must be 9.");

            _initialTiles = new Tile[tiles.Length];
            tiles.CopyTo(_initialTiles, 0);

            _currentTiles = new Tile[tiles.Length];
            tiles.CopyTo(_currentTiles, 0);
        }

        public int TilesCount => _initialTiles.Length;

        public string GetPositionUniqueId(int[] tilesOrder)
        {
            var builder = new StringBuilder();
            for (int i = 0; i < 9; i++)
            {
                builder.Append($"{_initialTiles[tilesOrder[i]].PositionId}");
            }
            return builder.ToString();
        }

        public bool IsSolved(IPuzzleIsSolvedStrategy strategy, int[] order)
        {
            if(strategy == null)
                throw new ArgumentNullException(nameof(strategy));

            return strategy.IsSolved(this, order);
        }

        public bool IsSolved(int[] order)
        {
            return new DefaultIsSolvedStrategy().IsSolved(this, order);
        }

        public bool IsSolved()
        {
            return new DefaultIsSolvedStrategy().IsSolved(this);
        }

        public Tile GetTile(int index) => index switch
        {
            >= 0 and <= 8 => _currentTiles[index],
            _ => throw new ArgumentOutOfRangeException(nameof(index), "Index must be between 0 and 8."),
        };

        public void Permutate(int[] order)
        {
            var temp = new Tile[_initialTiles.Length];

            for (int j = 0; j < order.Length; j++)
            {
                temp[j] = _initialTiles[order[j]];
            }

            _currentTiles = temp;
        }

        public int[] GetDefaultOrder()
        {
            // all tiles considered as matrix 
            // 0 1 2
            // 3 4 5
            // 6 7 8

            return Enumerable.Range(0, TilesCount).ToArray();
        }
    }
}
