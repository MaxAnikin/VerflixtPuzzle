using System.Text;

namespace VerflixtPuzzle.Model
{
    public class Tile(Side up, Side down, Side left, Side right)
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
        private readonly Tile[] _tiles;

        public Puzzle(Tile[] tiles)
        {
            if(tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles must be 9.");

            foreach (var tile in tiles)
            {
                if(tile == null)
                    throw new ArgumentNullException(nameof(tiles), "Tiles collection should not contain null Tiles.");
            }

            _tiles = tiles;
        }

        public void Reposition(int index1, int index2)
        {
            if (index1 is >= 0 and <= 8)
                throw new ArgumentOutOfRangeException(nameof(index1), "Index must be between 0 and 8.");

            if (index2 is >= 0 and <= 8)
                throw new ArgumentOutOfRangeException(nameof(index2), "Index must be between 0 and 8.");

            if(index1 == index2)
                return;

            (_tiles[index1], _tiles[index2]) = (_tiles[index2], _tiles[index1]);
        }

        public string GetPositionUniqueId()
        {
            var builder = new StringBuilder();
            for (int i = 0; i < 9; i++)
            {
                builder.Append($"{_tiles[i].PositionId}");
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
            >= 0 and <= 8 => _tiles[index],
            _ => throw new ArgumentOutOfRangeException(nameof(index), "Index must be between 0 and 8."),
        };
    }
}
