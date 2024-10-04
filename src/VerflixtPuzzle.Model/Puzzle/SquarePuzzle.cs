namespace VerflixtPuzzle.Model.Puzzle
{
    internal class SquarePuzzle : Puzzle
    {
        public SquarePuzzle(SquareTile[] tiles) : base(tiles)
        {
            if(tiles.Length != 9)
                throw new ArgumentException(nameof(tiles));

            BuildRelationships(tiles);
        }

        private void BuildRelationships(SquareTile[] tiles)
        {
            
        }
    }
}
