namespace VerflixtPuzzle.Model;

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