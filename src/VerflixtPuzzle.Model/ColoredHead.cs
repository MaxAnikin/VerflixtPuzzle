using System.Drawing;

namespace VerflixtPuzzle.Model
{
    internal class ColoredHead : ColoredSide
    {
        public ColoredHead(KnownColor color) : base(color)
        {
        }

        public override bool Fit(Side side)
        {
            var tail = side as ColoredTail;

            if (tail == null)
                return false;

            return tail.Color == Color;
        }
    }
}
