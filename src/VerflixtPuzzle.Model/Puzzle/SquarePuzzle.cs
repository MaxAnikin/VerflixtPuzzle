namespace VerflixtPuzzle.Model.Puzzle
{
    public class SquarePuzzle : Puzzle<SquareTile2>
    {
        private readonly SquareTile2[] _tiles;
        private readonly Dictionary<int[], bool> _crossResultCache = new Dictionary<int[], bool>();

        public SquarePuzzle(SquareTile2[] tiles)
        {
            if (tiles == null)
                throw new ArgumentNullException(nameof(tiles));

            _tiles = new SquareTile2[tiles.Length];

            tiles.CopyTo(_tiles, 0);
        }

        public SquareTile2 GetTile(int index)
        {
            if (index < 0 || index >= _tiles.Length)
                throw new ArgumentOutOfRangeException(nameof(index), $"Index must be between 0 and {_tiles.Length}.");
            
            return _tiles[index];
        }

        public bool Solve(Solution solution)
        {
            int[] crossId = GetCrossId(solution);
            var hasBeenSolved = _crossResultCache.TryGetValue(crossId, out var crossSolved);

            if (!hasBeenSolved)
            {
                crossSolved = SolveCross(solution);
                AddCrossToCache(solution, crossSolved);
            }

            if (!crossSolved)
                return false;

            return SolveRest(solution);
        }

        private void AddCrossToCache(Solution solution, bool crossSolved)
        {
            _crossResultCache.Add([solution.Order[0], solution.Order[1], solution.Order[2], solution.Order[3], solution.Order[4], solution.Order[5], solution.Order[6], solution.Order[7], solution.Order[8]], crossSolved);
            _crossResultCache.Add([solution.Order[6], solution.Order[3], solution.Order[0], solution.Order[7], solution.Order[4], solution.Order[1], solution.Order[8], solution.Order[5], solution.Order[2]], crossSolved);
            _crossResultCache.Add([solution.Order[8], solution.Order[7], solution.Order[6], solution.Order[5], solution.Order[4], solution.Order[3], solution.Order[2], solution.Order[1], solution.Order[0]], crossSolved);
            _crossResultCache.Add([solution.Order[2], solution.Order[5], solution.Order[8], solution.Order[1], solution.Order[4], solution.Order[7], solution.Order[0], solution.Order[3], solution.Order[6]], crossSolved);
        }

        private bool SolveCross(Solution solution)
        {
            if (solution == null) throw new ArgumentNullException(nameof(solution));

            var centerTile = GetTile(solution.Order[4]);
            var centerTileRotation = solution.Rotation[4];

            var upperTile = GetTile(solution.Order[1]);
            var upperTileRotation = solution.Rotation[1];

            if (!centerTile.GetSide((int)SquareTileSide.Up, centerTileRotation).Fit(upperTile.GetSide((int)SquareTileSide.Down, upperTileRotation)))
                return false;

            var leftTile = GetTile(solution.Order[3]);
            var leftTileRotation = solution.Rotation[3];

            if (!centerTile.GetSide((int)SquareTileSide.Left, centerTileRotation).Fit(leftTile.GetSide((int)SquareTileSide.Right, leftTileRotation)))
                return false;

            var rightTile = GetTile(solution.Order[5]);
            var rightTileRotation = solution.Rotation[5];

            if (!centerTile.GetSide((int)SquareTileSide.Right, centerTileRotation).Fit(rightTile.GetSide((int)SquareTileSide.Left, rightTileRotation)))
                return false;

            var downTile = GetTile(solution.Order[7]);
            var downTileRotation = solution.Rotation[7];

            if (!centerTile.GetSide((int)SquareTileSide.Down, centerTileRotation).Fit(downTile.GetSide((int)SquareTileSide.Up, downTileRotation)))
                return false;

            return true;
        }

        private int[] GetCrossId(Solution solution)
        {
            return [solution.Order[1], solution.Order[3], solution.Order[4], solution.Order[5], solution.Order[7]];
        }

        private bool SolveRest(Solution solution)
        {
            throw new NotImplementedException();
        }
    }
}
