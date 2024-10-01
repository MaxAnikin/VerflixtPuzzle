namespace VerflixtPuzzle.Model
{
    public sealed class Puzzle
    {
        private readonly Tile[] _initialTiles;

        public Puzzle(Tile[] tiles)
        {
            if(tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            _initialTiles = new Tile[tiles.Length];
            tiles.CopyTo(_initialTiles, 0);
        }

        public int TilesCount => _initialTiles.Length;

        public Tile GetTile(int index)
        {
            if (index < 0)
                throw new ArgumentOutOfRangeException(nameof(index), "Index must be positive.");

            return _initialTiles[index];
        }
    }
}
