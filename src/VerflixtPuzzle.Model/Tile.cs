namespace VerflixtPuzzle.Model;

public enum SquareTileSide : int
{
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

public class SquareTile2 : NSideTile
{
    public SquareTile2(ISide[] sides) : base(sides)
    {
        if (sides.Length != 4)
            throw new ArgumentOutOfRangeException(nameof(sides), "Square tile can have exactly 4 sides only.");
    }
}

public class SquareTile(ISide[] sides) : NSideTile(sides)
{
    public ISide Up => GetSide(1, Position);
    public ISide Down => GetSide(3, Position);
    public ISide Left => GetSide(0, Position);
    public ISide Right => GetSide(2, Position);

    public int Position { get; private set; } = 0;

    public void Rotate()
    {
        // clockwise rotation
        UpdatePosition(1);
    }

    private void UpdatePosition(int direction)
    {
        Position += direction;

        if (Position == Sides.Length)
            Position = 0;

        if (Position == -1)
            Position = Sides.Length - 1;
    }
}

public class NSideTile(ISide[] sides)
{
    protected readonly ISide[] Sides = sides;

    public ISide GetSide(int sideIndex, int rotation)
    {
        if (rotation < 0 || rotation > Sides.Length)
            throw new ArgumentOutOfRangeException(nameof(rotation));

        var i = sideIndex + rotation;
        if (i > Sides.Length - 1)
            i -= Sides.Length;

        return Sides[i];
    }
}