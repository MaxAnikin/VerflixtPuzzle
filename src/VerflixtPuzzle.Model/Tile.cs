namespace VerflixtPuzzle.Model;

public class Tile
{
    public ISide Left { get; set; }
    public ISide Up { get; set; }
    public ISide Right { get; set; }
    public ISide Down { get; set; }
    public int Position { get; private set; } = 0;

    public Tile(ISide left, ISide up, ISide right, ISide down)
    {
        Left = left;
        Up = up;
        Right = right;
        Down = down;
        Position = 0;
    }

    public void Rotate()
    {
        // clockwise rotation

        var u = Up;
        Up = Left;
        Left = Down;
        Down = Right;
        Right = u;

        UpdatePosition(1);
    }

    private void UpdatePosition(int direction)
    {
        Position += direction;

        if (Position == 4)
            Position = 0;

        if (Position == -1)
            Position = 3;
    }
}