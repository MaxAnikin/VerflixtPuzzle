using System.Text;
using VerflixtPuzzle.Model.TSide;

namespace VerflixtPuzzle.Model
{
    public class Tile(Side left, Side up, Side right, Side down)
    {
        public Side Up { get; private set; } = up ?? throw new ArgumentNullException(nameof(up));
        public Side Down { get; private set; } = down ?? throw new ArgumentNullException(nameof(down));
        public Side Left { get; private set; } = left ?? throw new ArgumentNullException(nameof(left));
        public Side Right { get; private set; } = right ?? throw new ArgumentNullException(nameof(right));
        public int PositionId { get; private set; } = 0;

        public void Rotate()
        {
            // clockwise rotation

            var u = Up;
            Up = Left;
            Left = Down;
            Down = Right;
            Right = u;

            UpdatePositionId(1);
        }

        private void UpdatePositionId(int direction)
        {
            PositionId += direction;

            if (PositionId == 4)
                PositionId = 0;

            if (PositionId == -1)
                PositionId = 3;
        }
    }

    public sealed class Puzzle
    {
        private Tile[] _initialTiles;

        public Puzzle(Tile[] tiles)
        {
            if(tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles must be 9.");

            _initialTiles = tiles;
        }

        public int TilesCount => _initialTiles.Length;

        public string GetPositionUniqueId()
        {
            var builder = new StringBuilder();
            for (int i = 0; i < 9; i++)
            {
                builder.Append($"{_initialTiles[i].PositionId}");
            }
            return builder.ToString();
        }

        public bool IsSolved(IPuzzleIsSolvedStrategy strategy)
        {
            if(strategy == null)
                throw new ArgumentNullException(nameof(strategy));

            return strategy.IsSolved(this);
        }

        public bool IsSolved()
        {
            return new DefaultIsSolvedStrategy().IsSolved(this);
        }

        public Tile GetTile(int index) => index switch
        {
            >= 0 and <= 8 => _initialTiles[index],
            _ => throw new ArgumentOutOfRangeException(nameof(index), "Index must be between 0 and 8."),
        };

        public void Permutate(int[] order)
        {
            var temp = new Tile[_initialTiles.Length];
            for (int j = 0; j < order.Length; j++)
            {
                temp[j] = _initialTiles[order[j]];
            }

            _initialTiles = temp;
        }
    }
}
