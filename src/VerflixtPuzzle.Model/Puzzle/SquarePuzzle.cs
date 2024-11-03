namespace VerflixtPuzzle.Model.Puzzle
{
    public class SquarePuzzle : Puzzle<SquareTile>
    {
        private readonly SquareTile[] _tiles;
        private Dictionary<int[], bool> _crossResultCache;

        public SquarePuzzle(SquareTile[] tiles)
        {
            if (tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            _tiles = new SquareTile[tiles.Length];

            tiles.CopyTo(_tiles, 0);
        }

        public SquareTile GetTile(int index)
        {
            if (index < 0 || index >= _tiles.Length)
                throw new ArgumentOutOfRangeException(nameof(index), $"Index must be between 0 and {_tiles.Length}.");
            
            return _tiles[index];
        }

        public bool Solved(Solution solution)
        {
            int[] crossId = GetCrossId(solution);
            if(_crossResultCache.TryGetValue(crossId, out var crossResult) && !crossResult)
                return false;

            

            return RestSolved(solution);
        }

        private int[] GetCrossId(Solution solution)
        {
            throw new NotImplementedException();
        }

        private bool RestSolved(Solution solution)
        {
            throw new NotImplementedException();
        }

        private SquareTile GetLeftTile(Solution solution)
        {
            throw new NotImplementedException();
        }

        private SquareTile GetCenterTile(Solution solution)
        {
            return _tiles[GetCenterTileIndex(solution)];
        }

        private int GetCenterTileIndex(Solution combination)
        {
            if (combination is null)
            {
                throw new ArgumentNullException(nameof(combination));
            }

            return combination.Order[4];
        }
    }
}
