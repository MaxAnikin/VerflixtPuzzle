namespace VerflixtPuzzle.Model;

public class SquareTile(ISide[] sides) : NSideTile(sides)
{
    public ISide Up => GetSide(1, PositionId);
    public ISide Down => GetSide(3, PositionId);
    public ISide Left => GetSide(0, PositionId);
    public ISide Right => GetSide(2, PositionId);

    public int PositionId { get; private set; } = 0;

    public void Rotate()
    {
        // clockwise rotation
        UpdatePositionId(1);
    }

    private void UpdatePositionId(int direction)
    {
        PositionId += direction;

        if (PositionId == 4)
            PositionId = 0;

        if (PositionId == -1)
            PositionId = 3;
    }
}

public class NSideTile(ISide[] sides)
{
    public ISide GetSide(int index, int position)
    {
        if (position < 0 || position > sides.Length)
            throw new ArgumentOutOfRangeException(nameof(position));

        return sides[index + position];
    }
}