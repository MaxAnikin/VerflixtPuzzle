namespace VerflixtPuzzle.Model;

public class Tile(ISide left, ISide up, ISide right, ISide down)
{
    public ISide Up { get; private set; } = up ?? throw new ArgumentNullException(nameof(up));
    public ISide Down { get; private set; } = down ?? throw new ArgumentNullException(nameof(down));
    public ISide Left { get; private set; } = left ?? throw new ArgumentNullException(nameof(left));
    public ISide Right { get; private set; } = right ?? throw new ArgumentNullException(nameof(right));
    public int PositionId { get; private set; } = 0;

    public void Rotate()
    {
        // clockwise rotation

        var u = Up;
        Up = Left;
        Left = Down;
        Down = Right;
        Right = u;

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