using System.Drawing;

namespace VerflixtPuzzle.Model
{
    public abstract class ColoredSide(KnownColor color) : Side
    {
        public KnownColor Color { get; } = color;
    }
}
