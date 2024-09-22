namespace VerflixtPuzzle.Model
{
    public record Tile(Side Up, Side Down, Side Left, Side Right);

    public class Puzzle
    {
        private Tile[] _tiles;

        private TilesOrder CurrentOrder { get; }

        public Puzzle(Tile[] tiles)
        {
            if(tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles must be 9.");

            _tiles = tiles;
        }

    }
}
