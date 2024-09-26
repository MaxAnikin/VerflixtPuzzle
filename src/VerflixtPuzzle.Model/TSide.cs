namespace VerflixtPuzzle.Model.TSide
{
    public abstract record Side
    {
        public abstract bool Fit(Side side);
    }

    public record TortoiseSide(Shape Body, System.Drawing.Color Color) : Side
    {
        public override bool Fit(Side side)
        {
            if(side is TortoiseSide tortoiseSide)
                return tortoiseSide.Body != Body && tortoiseSide.Color == Color;

            return false;
        }
    }

    public enum Shape
    {
        Tail,
        Head
    }
}