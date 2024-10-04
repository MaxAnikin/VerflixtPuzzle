namespace VerflixtPuzzle.Model
{
    internal class TilesOrder
    {
        private SquareTile[] _tiles;

        public TilesOrder(SquareTile[] tiles)
        {
            if (tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            if (tiles.Length != 9)
                throw new ArgumentOutOfRangeException(nameof(tiles), "Number of tiles must be 9.");

            _tiles = tiles;
        }
    }
}