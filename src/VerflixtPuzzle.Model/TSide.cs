namespace VerflixtPuzzle.Model.TSide
{
    public record struct TortoiseSide(Shape Body, System.Drawing.Color Color) : ISide
    {
        public bool Fit(ISide side)
        {
            if(side is TortoiseSide tortoiseSide)
                return Fit(tortoiseSide);

            return false;
        }

        public bool Fit(TortoiseSide side)
        {
            return side.Body != Body && side.Color == Color;
        }
    }

    public enum Shape
    {
        Tail,
        Head
    }
}