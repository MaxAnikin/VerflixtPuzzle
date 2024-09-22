namespace VerflixtPuzzle.Model
{
    public abstract class Side : ISide
    {
        public abstract bool Fit(Side part);
    }
}