namespace VerflixtPuzzle.Model
{
    internal class TilesOrder
    {
        private Tile[] _tiles;

        public TilesOrder(Tile[] tiles)
        {
            if (tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles must be 9.");

            _tiles = tiles;
        }
    }
}